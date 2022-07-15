pub mod tcp {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;
    use std::error::Error;

    fn this() -> TcpClient {
        let ip = "IP_TRIM5";
        let plc_ip = crate::read_env::env::read_str(ip);
        let mut client = TcpClient::new(&plc_ip.unwrap_or_default());
        client
    }
    /// Communication session with the PLC via Modbus TCP
    pub fn client(client: &mut TcpClient, func: fn(&mut TcpClient)) {
        let result = client.connect();
        match result {
            Err(message) => {
                // Create event "app connection error to PLC".
                // and records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::alarm::info::event_err_connect_to_plc(&message);
            }
            Ok(_) => {
                info!("app communication with plc: ok");
                // Reading variable values from the PLC "trim5" via Modbus TCP
                // and writing the obtained values to the PostgreSQL DBMS.
                func(client);
                client.disconnect();
            }
        }
    }
}