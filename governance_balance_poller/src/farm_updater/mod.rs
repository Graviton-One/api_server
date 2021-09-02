use crate::schema::gton_price;
use diesel::{
    sql_types::*,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use tokio::time::{
    sleep,
  Duration,
};
use std::sync::Arc;
use crate::schema::{
    gton_farms,
};

use ethcontract::web3::{
    self,
    contract::{Contract, Options},
    types::*,
};

pub type Web3Instance = web3::Web3<ethcontract::Http>;

pub fn prepare_amount(reserve: U256, dec: i64) -> f64 {
    reserve.to_f64_lossy() / 10_f64.powf(dec as f64)
}

pub fn create_instance(rpc_url: &str) -> Web3Instance {
    let http = web3::transports::Http::new(String::from(rpc_url).as_str())
        .expect("error creating web3 instance");
    web3::Web3::new(http)
}

pub async fn get_total_supply(web3: &Web3Instance, pool: Address) -> f64 {
    let contract = Contract::from_json(
        web3.eth(),
        pool,
        include_bytes!("./abi/erc20.json"),
    ).expect("error contract creating");
    let res = contract
        .query("totalSupply", (), None, Options::default(), None).await.unwrap();
    prepare_amount(res, 18)
}

pub async fn count_farm_users(token_id: U256, ftm_web3: &Web3Instance, lp_keeper: Address) -> i32 {
    let contract = Contract::from_json(
        ftm_web3.eth(),
        lp_keeper,
        include_bytes!("./abi/lpkeeper.json"),
    ).expect("error contract creating");
    contract
        .query("totalTokenUsers", (token_id,), None, Options::default(), None).await.unwrap()
}

pub async fn farmed_from_farm(ftm_web3: &Web3Instance, farm_address: Address) -> f64 {
    let contract = Contract::from_json(
        ftm_web3.eth(),
        farm_address,
        include_bytes!("./abi/linear.json"),
    ).expect("error contract creating");
    let res = contract
        .query("totalUnlocked", (), None, Options::default(), None).await.unwrap();
    prepare_amount(res, 18)
}

pub async fn get_locked_amount(web3: &Web3Instance, pool: Address, lock: Address) -> f64 {
    let contract = Contract::from_json(
        web3.eth(),
        pool,
        include_bytes!("./abi/erc20.json"),
    ).expect("error contract creating");
    let res = contract
        .query("balanceOf", lock, None, Options::default(), None).await.unwrap();
        prepare_amount(res, 18)
}

pub fn calc_lp_price(tvl: f64, total_supply: f64) -> f64 {
    tvl / total_supply
}

pub fn calc_lp_liquidity(lp_price: f64, lp_locked: f64) -> f64 {
    lp_price * lp_locked
}

pub fn calculate_apy(total_locked: f64, gton_price: f64, amount_per_day: i64) -> f64 {
    // total earn per year / total locked on contract
    // all values are compatible to dollar value
    (365 * amount_per_day) as f64 * gton_price / total_locked * 100 as f64
}

#[derive(Insertable,Queryable,Clone,Debug,AsChangeset)]
#[table_name = "gton_farms"]
pub struct UpdateFarm {
    id: i64,
    farmed: f64,
    apy: f64,
    addresses_in: i32,
    lp_price: f64,
    liquidity: f64
}

#[derive(Default, Debug, Clone, QueryableByName)]
pub struct FarmsData {
    #[sql_type = "BigInt"]
    id: i64,
    #[sql_type = "Float8"]
    tvl: f64,
    #[sql_type = "Text"]
    node_url: String,
    #[sql_type = "Text"]
    pool_address: String,
    #[sql_type = "Integer"]
    token_id: i32,
    #[sql_type = "Text"]
    lock_address: String,
    #[sql_type = "Text"]
    farm_linear: String,
}


impl FarmsData {
    fn get_farms(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Vec<FarmsData> {
        diesel::sql_query("SELECT f.id, p.tvl, p.pool_address, c.node_url, f.lock_address, f.farm_linear, f.token_id 
        FROM gton_farms AS f
        LEFT JOIN pools AS p ON p.id = f.pool_id
        LEFT JOIN dexes AS d ON d.id = p.dex_id 
        LEFT JOIN chains AS c ON c.id = d.chain_id;
        ").get_results::<FarmsData>(&conn.get().unwrap())
        .unwrap()
    }
    fn get_gton_price(conn: &PgConnection) -> f64 {
         gton_price::table
            .select(gton_price::price)
            .order_by(gton_price::market_time.asc())
            .limit(1)
            .get_result::<f64>(conn)
            .unwrap()
    }
    fn update_farm_data(
        data: UpdateFarm,
        conn: Arc<Pool<ConnectionManager<PgConnection>>>
    ) -> () {
        diesel::update(gton_farms::table)
        .filter(gton_farms::id.eq(data.id))
        .set(data)
        .execute(&conn.get().unwrap())
        .unwrap();
    }
}

pub struct FarmUpdater {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    pgconn: PgConnection,
    lp_keeper: Address
}

pub fn string_to_h160(string: String) -> ethcontract::H160 {
    ethcontract::H160::from_slice(String::from(string).as_bytes())
}

impl FarmUpdater {
    pub fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("missing db url");
        let lp_keeper: Address = "0xA0447eE66E44BF567FF9287107B0c3D2F88efD93".parse().unwrap();
        let manager = ConnectionManager::<PgConnection>::new(
            &db_url,
        );
        let pgconn = PgConnection::establish(&db_url).unwrap();

        let pool = Pool::builder().build(manager).expect("pool build");

        let pool = std::sync::Arc::new(pool);
        FarmUpdater {
            pgconn,
            pool,
            lp_keeper,
        }
    }
    pub async fn update_farms(&self) -> () {
        let ftm_web3 = create_instance("https://rpcapi.fantom.network");
        loop {
        let farms: Vec<FarmsData> = FarmsData::get_farms(self.pool.clone());
        let gton_price = FarmsData::get_gton_price(&self.pgconn);
        for farm in farms {
            let web3 = create_instance(&farm.node_url);
            let pool_address: Address = farm.pool_address.parse().unwrap();
            let locked = get_locked_amount(&web3,  pool_address, farm.lock_address.parse().unwrap()).await;
            println!("locked {}", locked);
            let farmed: f64 = farmed_from_farm(&ftm_web3, farm.farm_linear.parse().unwrap()).await;
            let total_supply = get_total_supply(&web3, pool_address).await;
            println!("total  {}", total_supply);
            let lp_price: f64 = calc_lp_price(farm.tvl, total_supply);
            println!("id {}", farm.token_id);
            println!("lpkeeper {}", &self.lp_keeper);
            let liquidity = calc_lp_liquidity(lp_price, locked);
            let token_id: U256 = U256::from(farm.token_id);
            let addresses_in = count_farm_users(token_id, &ftm_web3, self.lp_keeper).await;
            let apy = calculate_apy(liquidity, gton_price, 10);
            FarmsData::update_farm_data(UpdateFarm {
                id: farm.id,
                farmed,
                lp_price,
                addresses_in,
                apy,
                liquidity
            }, self.pool.clone())
            }
        sleep(Duration::from_secs((15) as u64)).await;
        }
    }
}
