/* extern crate pcsc;
extern crate bytes;

use pcsc::*;
use bytes::*;

pub struct MyCard {
    pub mut card: pcsc::Card;
    pub mut ctx: pcsc::Context;
    //pub mut reader: 
}

impl MyCard {

    pub fn new() -> MyCard {

        //let app = Application { app_id: mycard::MyCard::APP_ID_CARD_MANAGEMENT.to_vec() };
        // let current_application: Vec<u8> = mycard::MyCard::APP_ID_CARD_MANAGEMENT.to_vec();

        let ctx = match Context::establish(Scope::User) {
            Ok(ctx) => ctx,
            Err(err) => {
                eprintln!("Failed to establish context: {}", err);
                std::process::exit(1);
            }
        };

        // List available readers.
        let mut readers_buf = [0; 2048];
        let mut readers = match ctx.list_readers(&mut readers_buf) {
            Ok(readers) => readers,
            Err(err) => {
                eprintln!("Failed to list readers: {}", err);
                std::process::exit(1);
            }
        };

        // Use the first reader.
        let reader = match readers.next() {
            Some(reader) => reader,
            None => {
                println!("No readers are connected.");
                return;
            }
        };
        println!("Using reader: {:?}", reader);

        // Connect to the card.
        let my_card = match ctx.connect(reader, ShareMode::Shared, Protocols::ANY) {
            Ok(card) => card,
            Err(Error::NoSmartcard) => {
                println!("A smartcard is not present in the reader.");
                return;
            }
            Err(err) => {
                eprintln!("Failed to connect to card: {}", err);
                std::process::exit(1);
            }
        };

        return MyCard {
            card: my_card
            ctx: 
        }
    }

    pub const TAG_ID_CARD_NUMBER: u8 = 1;
    pub const TAG_ID_CERTIFICATE_SERIAL_NUMBER: u8 = 2;            // Q: Why 2 and not 0x02? 
    pub const TAG_ID_KEY_KCV: u8 = 0xC0;
    pub const TAG_ID_KEY_COUNTER: u8 = 0xC1;

    pub const TAG_ID_DOK_STATE: u8 = 0x8B;
    pub const TAG_ID_DOK_TRY_LIMIT: u8 = 0x8C;
    pub const TAG_ID_DOK_MAX_TRY_LIMIT: u8 = 0x8D;

    pub const TAG_ID_IOK_STATE: u8 = 0x82;
    pub const TAG_ID_IOK_TRY_LIMIT: u8 = 0x83;
    pub const TAG_ID_IOK_MAX_TRY_LIMIT: u8 = 0x84;
    
    // Q: Why are those private in Java?
    pub const APP_ID_CARD_MANAGEMENT: [u8; 9] = [0xD2, 0x03, 0x10, 0x01, 0x00, 0x01, 0x00, 0x02, 0x02];
    pub const APP_ID_FILE_MANAGEMENT: [u8; 10] = [0xD2, 0x03, 0x10, 0x01, 0x00, 0x01, 0x03, 0x02, 0x01, 0x00];

    pub const FILE_ID_CERTIFICATE_AUTHORIZATION: u32 = 0x0132;         // Q: Does the int size matter, beyond satisfying the minimal needed size?
    pub const FILE_ID_CERTIFICATE_IDENTIFICATION: u32 = 0x0001;
}

// pub struct ICardInterface { }

impl MyCard {
    /*
    pub fn new() {
        MyCard {            
        }
    }
    */

    // pub fn select_application(app_id: [u8; 8]) -> bool {

/* public boolean selectApplication(byte[] appId) throws CardException {
        if (currentApplication != null) {
            if (Arrays.equals(currentApplication, appId)) { //Don't select application if is already set
                return true;
            }
        }

        byte[] selectApplet = new byte[]{
                0x00, (byte) 0xA4, 0x04, 0x0C, (byte) appId.length,
        };
        IResponseAPDU r = c.transmit(c.createCommand((HexUtils.concatArrays(selectApplet, appId))));

        if (r.getSW() == 0x9000) {
            currentApplication = appId;
            return true;
        }
        return false;
    }
    */

    // Q: Why this isn't a const? Do the different app_id lenths differ?
    // let cmd_select_applet = [0x00, 0xA4, 0x0C, app_id.len() as u8];
    //let response_apdu = self.card.transmit(self.card.createCommand((HexUtils.concatArrays(select_applet_cmd, appId))));

    pub fn transmit(command: Vec<u8>) {        // TODO: Parameter to the ICommandAPDU type

    }

    pub fn create_command(command_data: Vec<u8>) {                  // TODO: How long commandData array? Or bytes? Or Vec<u8>?

    }

    pub fn get_request_command() -> Vec<u8> {
        vec![0x00] 
    }

    pub fn get_ATR() -> Vec<u8> {       // TODO: How long array?
        vec![0x00]
    }

    pub fn get_SW() -> u8 {
        0x00
    }

    pub fn get_SW1() -> u8 {
        0x00
    }

    pub fn get_SW2() -> u8 {
        0x00
    }

    pub fn getData(&self, tag_id: u8, auth_id: u8) -> Vec<u8> {
        /*
        let mut authId = authId << 4;
        let request_command = [
            0x80,
            0xCA,
            (authId | 1) as u8,
            (authId | tagId) as u8,
            0
        ];

        let response_data = self.transmit(self.create_command(request_command));

        return response_data;
        */

        return vec![0x00]

        // TODO: Put the 0x9000 result in a legible constant like RESULT_OK
        /* if (r.getSW() == 0x9000) {
            return r.getData();
        } else if (r.getSW1() == 0x6c) {
            request[request.length - 1] = (byte) r.getSW2();
            r = c.transmit(c.createCommand(request));
            if (r.getSW() == 0x9000) {
                return r.getData();
            }
        }
        return Err("Error"); */
    }
}

*/