fn main() {
    let logging: u32 = 5;
    println!("{logging}");
    let res: u32 = 35;
    println!("{res}");
    let log: u32 = res.ilog(logging);
    println!("{log}");
    let powed: u32 = logging.pow(log);
    println!("{powed}");
    assert!(res % logging == 0);
    assert_eq!(res, powed);
}
