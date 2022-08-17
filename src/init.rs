pub mod postgresql {
    use core::panic;

    /// Setting the transaction isolation level
    /// creating tables in Postgresql.
    pub fn init_postgres() {
        // Setting the transaction isolation level to serializable.
        match crate::psql::postgresql::set_transaction_isolation() {
            Ok(_) => info!("set_transaction_isolation(): ok"),
            Err(e) => {
                info!("{}", e);
                panic!("{}", e)
            }
        }

        // Creating avr_control_insert_table
        // a table containing information about the operation of the generator.
        match crate::psql::postgresql::create_ats_control_table() {
            Ok(_) => info!("create_avr_control_insert_table(): ok"),
            Err(e) => {
                info!("{}", e);
                panic!("{}", e)
            }
        }

        // Creating log_of_work_app_table
        // a table containing information about
        // the operation of the application (intended for the end user).
        match crate::psql::postgresql::create_app_log_table() {
            Ok(_) => info!("create_log_of_work_app_table(): ok"),
            Err(e) => {
                info!("{}", e);
                panic!("{}", e)
            }
        }

        // Creating winter_garden_table
        // a table containing information about
        // the operation of the automated winter garden management system.
        match crate::psql::postgresql::create_winter_garden_table() {
            Ok(_) => info!("create_winter_garden_table(): ok"),
            Err(e) => {
                info!("{}", e);
                panic!("{}", e)
            }
        }

        // Creating generator_load_table
        // a table containing information about
        // the load level connected to the generator.
        match crate::psql::postgresql::create_generator_load_table() {
            Ok(_) => info!("create_generator_load_table(): ok"),
            Err(e) => {
                info!("{}", e);
                panic!("{}", e)
            }
        }

        match crate::psql::postgresql::create_tg_message_table() {
            Ok(_) => info!("create_tg_message_table(): ok"),
            Err(e) => {
                info!("{}", e);
                panic!("{}", e)
            }
        }

        match crate::psql::postgresql::create_tg_chat_table() {
            Ok(_) => info!("create_tg_chat_table(): ok"),
            Err(e) => {
                info!("{}", e);
                panic!("{}", e)
            }
        }
    }
}
