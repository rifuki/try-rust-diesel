use color_print::{cformat, cprintln};
use diesel::PgConnection;
use rust_go::{
    crud::code_name::{create::create_code_name, reads::read_all_code_names, delete::delete_code_name},
    utils::{
        cli::{clear_screen, get_user_input, SEPARATOR_EQ},
        database::establish_connection,
    },
};

fn print_menu() {
    println!("==>           Menu           <==");
    cprintln!("<g>1. Show All Vocaloid Characters</g>");
    cprintln!("<g>5. Show All Code-Names</g>");
    cprintln!("<b>6. Insert Code-Name</b>");
    cprintln!("<r>7. Delete Code-Name</r>");
    cprintln!("<k>8. Exit</k>");
    cprintln!("{}", SEPARATOR_EQ);
}

fn wait_for_return() {
    get_user_input(&cformat!("\nPress <k><s>Enter</s></k> to continue..."));
}

fn handle_user_input(user_input: &str, conn: &mut PgConnection) -> bool {
    match user_input {
        "1" => {
            clear_screen();
            cprintln!("<c><s>Showing All Vocaloid Characters</s></c>");
            wait_for_return();
            clear_screen();
        }
        "5" => {
            clear_screen();
            read_all_code_names(conn);
            wait_for_return();
            clear_screen();
        }
        "6" => {
            clear_screen();
            create_code_name(conn);
            wait_for_return();
            clear_screen();
        }
        "7" => {
            clear_screen();
            delete_code_name(conn);
            wait_for_return();
            clear_screen();
        }
        "8" => {
            clear_screen();
            cprintln!("<c><b>Bye-bye~</b></c>");
            return false;
        }
        _ => cprintln!("<r><s>Invalid menu!</s></r> Please enter a valid menu.\n"),
    }
    true
}

fn main() {
    let mut conn = establish_connection();

    loop {
        print_menu();
        let user_input = get_user_input("Enter your choice: ");
        if !handle_user_input(&user_input, &mut conn) {
            break;
        }
    }
}
