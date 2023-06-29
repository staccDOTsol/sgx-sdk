pub mod binance;
pub mod instruction;

pub use switchboard_solana::prelude::*;

#[tokio::main(worker_threads = 12)]
async fn main() {
    // First, initialize the runner instance with a freshly generated Gramine keypair
    let runner = FunctionRunner::new_from_cluster(Cluster::Devnet, None)
        .map_err(|e| SwitchboardClientError::CustomError {
            message: "failed to load FunctionRunner".to_string(),
            source: Box::new(e),
        })
        .unwrap();

    // Then, write your own Rust logic and build a Vec of instructions.
    // Should  be under 700 bytes after serialization
    let (state_pubkey, _bump) =
        Pubkey::find_program_address(&[b"BINANCEORACLE"], &binance_oracle::ID);
    let symbols = ["BTCUSDT", "ETHUSDT", "ADAUSDT", "BNBUSDT", "XRPUSDT", "SOLUSDT", "DOTUSDT", "UNIUSDT", "LINKUSDT", "LTCUSDT", "BCHUSDT", "XLMUSDT", "DOGEUSDT", "ETCUSDT", "VETUSDT", "TRXUSDT", "EOSUSDT", "FILUSDT", "AAVEUSDT", "XTZUSDT", "THETAUSDT", "ATOMUSDT", "XMRUSDT", "NEOUSDT", "MIOTAUSDT", "MKRUSDT", "BUSDUSDT", "CROUSDT", "ALGOUSDT", "LEOUSDT", "BSVUSDT", "AVAXUSDT", "COMPUSDT", "HTUSDT", "RUNEUSDT", "KLAYUSDT", "SNXUSDT", "FTTUSDT", "DCRUSDT", "ZECUSDT", "CHZUSDT", "EGLDUSDT", "WAVESUSDT", "NEARUSDT", "HOTUSDT", "HNTUSDT", "DASHUSDT", "ZILUSDT", "ENJUSDT", "GRTUSDT", "MANAUSDT", "RVNUSDT", "ZENUSDT", "BATUSDT", "ONEUSDT", "SUSHIUSDT", "KSMUSDT", "ICXUSDT", "BTGUSDT", "STXUSDT", "NEXOUSDT", "LUNAUSDT", "RSRUSDT", "DGBUSDT", "CAKEUSDT", "ONTUSDT", "BNTUSDT", "QTUMUSDT", "SCUSDT", "SXPUSDT", "UMAUSDT", "ZRXUSDT", "OKBUSDT", "KNCUSDT", "ANKRUSDT", "IOSTUSDT", "CELOUSDT", "LSKUSDT", "WAXPUSDT", "RENUSDT", "NANOUSDT", "CRVUSDT", "HUSDUSDT", "ARUSDT", "SKLUSDT", "BALUSDY"];
        
    let tickers: Vec<binance::Ticker> = binance::fetch_prices(symbols.to_vec()).await.unwrap();
    println!("{:#?}", tickers);
    let ixs: Vec<solana_program::instruction::Instruction> = symbols
        .iter()
        .enumerate()
        .map(|(i, s)| instruction::build(&runner, s, tickers.get(i).unwrap(), &state_pubkey))
        .collect();

    // Finally, emit the signed quote and partially signed transaction to the functionRunner oracle
    // The functionRunner oracle will use the last outputted word to stdout as the serialized result. This is what gets executed on-chain.
    runner.emit(ixs).await.unwrap();
}
