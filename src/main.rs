use std::io::{stdin, BufRead, BufReader};

use domain::Contact;
use rw::{read::{Query, UserProjection}, write::{CommandHandler, CreateUser, CreditUser, DebitUser, DeleteUser, UserAggregate}, UserProjector};

mod domain;
mod rw;
mod events;
fn main() -> anyhow::Result<()> {
    let mut aggregate = UserAggregate::new();
    let mut projector = UserProjector::new();


    menu();
    
    for line in BufReader::new(stdin()).lines() {
        let line = line?;

        if line.is_empty(){
            menu();
            continue;
        }

        let args = line.split_whitespace().collect::<Vec<&str>>();

        match args[..] {
            ["lister"] => {
                let users = UserProjection::handle(Query::UsersQuery, &projector.repository);
                if users.is_empty(){
                    println!("(vide)");
                }else{
                    for user in users {
                        println!("{user}");
                    }
                }

                
            },

            ["lister", "+", amount] => {
                let users = UserProjection::handle(Query::UsersWithMoneyAbove(amount.parse()?), &projector.repository);

                if users.is_empty(){
                    println!("(vide)");
                }else{
                    for user in users {
                        println!("{user}");
                    }
                }
            },

            ["lister", "-", amount] => {
                let users = UserProjection::handle(Query::UsersWithMoneyBelow(amount.parse()?), &projector.repository);

                if users.is_empty(){
                    println!("(vide)");
                }else{
                    for user in users {
                        println!("{user}");
                    }
                }
            },

            ["ajouter", name, mail] => {
                if let Some(event) = aggregate.handle(CreateUser(name.to_string(), Contact { mail: mail.to_string() })) {
                    projector.project(event);
                    println!("1 modification")
                }else{
                    println!("0 modification")
                }
                
            },

            ["supprimer", name] => {
                if let Some(event) = aggregate.handle(DeleteUser(name.to_string())) {
                    projector.project(event);
                    println!("1 modification");
                }else{
                    println!("0 modification");
                }
                
            },

            ["crediter", name, amount] => {
                if let Some(event) = aggregate.handle(CreditUser(name.to_string(), amount.parse()?)) {
                    projector.project(event);
                    println!("1 modification");
                }else{
                    println!("0 modification");
                }
                
            },

            ["debiter", name, amount] => {
                if let Some(event) = aggregate.handle(DebitUser(name.to_string(), amount.parse()?)){
                    projector.project(event);
                    println!("1 modification");
                }else{
                    println!("0 modification");
                }
            },

            
            _ => menu()
        }
        println!();
    }

    Ok(())
}

fn menu(){
    println!("Commandes:");
    println!("\t- lister");
    println!("\t- lister <+ | -> <montant>");
    println!("\t- ajouter <nom> <mail>");
    println!("\t- supprimer <nom>");
    println!("\t- crediter <nom> <montant>");
    println!("\t- debiter <nom> <montant>");
}
