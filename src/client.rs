use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:5050").await?;

    let req = tonic::Request::new(BtcPaymentRequest {
        from_addr: "69420".to_owned(),
        to_addr: "42069".to_owned(),
        amount: 69,
    });

    let res = client.send_payment(req).await?;

    println!("response : {:?}", res);

    Ok(())
}

