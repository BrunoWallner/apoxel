use tokio::net::TcpStream;

use std::error::Error;

use protocol::{header::Header, reader, writer};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("0.0.0.0:8000").await?;
    let (read, write) = stream.into_split();
    let mut reader = reader::Reader::new(read);
    let mut writer = writer::Writer::new(write);

    tokio::spawn(async move {
        let name = String::from("luca");
        writer.send_header(&Header::Register).await?;
        writer.send_string(name).await?;

        Ok::<_, tokio::io::Error>(())
    });

    tokio::spawn(async move {
        let header = reader.get_header().await?;
        match header {
            Header::ReceiveToken => {
                let token = reader.get_token().await?;
                println!("got token: {:?}", token);
            }
            Header::Error => {
                let error = reader.get_error().await?;
                println!("error occured: {:?}", error);
            }
            header => {
                println!("unsupported header: {:?}", header);
            }
        }
        Ok::<_, tokio::io::Error>(())
    });

    std::thread::park();
    Ok(())
}
