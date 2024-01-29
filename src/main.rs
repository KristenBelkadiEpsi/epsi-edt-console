use std::env;
use chrono::{DateTime, TimeZone, Utc};
use epsi_edt_console::recuperer_infos;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    let identifiant: &String = &args[1];
    let split_date: Vec<String> = args[2].split("/").map(|e: &str| e.to_string()).collect();
    let date: DateTime<Utc> = Utc
        .with_ymd_and_hms(
            split_date[2].clone().parse().unwrap(),
            (&split_date[1].clone()).parse().unwrap(),
            split_date[0].clone().parse().unwrap(),
            0,
            0,
            0,
        )
        .unwrap();
    let infos: (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ) = recuperer_infos(identifiant, &date).await;
    let mut taille_max_zero: usize = usize::MIN;
    let mut taille_max_un: usize = usize::MIN;
    let mut taille_max_deux: usize = usize::MIN;
    let mut taille_max_trois: usize = usize::MIN;
    let mut taille_max_quatre: usize = usize::MIN;

    for i in 0..infos.0.len() {
        let taille_zero: usize = infos.0[i].chars().count();
        let taille_un: usize = infos.1[i].chars().count();
        let taille_deux: usize = infos.2[i].chars().count();
        let taille_trois: usize = infos.3[i].chars().count();
        let taille_quatre: usize = infos.4[i].chars().count();

        taille_max_zero = taille_max_zero.max(taille_zero);
        taille_max_un = taille_max_un.max(taille_un);
        taille_max_deux = taille_max_deux.max(taille_deux);
        taille_max_trois = taille_max_trois.max(taille_trois);
        taille_max_quatre = taille_max_quatre.max(taille_quatre);
    }
    for i in 0..infos.0.len() {
        let taille_zero: usize = infos.0[i].chars().count();
        let taille_un: usize = infos.1[i].chars().count();
        let taille_deux: usize = infos.2[i].chars().count();
        let taille_trois: usize = infos.3[i].chars().count();
        let taille_quatre: usize = infos.4[i].chars().count();

        let chaine = format!(
            "║ {}{} ║ {}{} ║ {}{} ║ {}{} ║ {}{} ║",
            infos.0[i],
            " ".repeat(taille_max_zero - taille_zero),
            infos.1[i],
            " ".repeat(taille_max_un - taille_un),
            infos.2[i],
            " ".repeat(taille_max_deux - taille_deux),
            infos.3[i],
            " ".repeat(taille_max_trois - taille_trois),
            infos.4[i],
            " ".repeat(taille_max_quatre - taille_quatre)
        );
        let ligne_dessus_zero: String = format!("╔{}╦", "═".repeat(taille_max_zero + 2));
        let ligne_dessus_un: String = format!("{}╦", "═".repeat(taille_max_un + 2));
        let ligne_dessus_deux: String = format!("{}╦", "═".repeat(taille_max_deux + 2));
        let ligne_dessus_trois: String = format!("{}╦", "═".repeat(taille_max_trois + 2));
        let ligne_dessus_quatre: String = format!("{}╗", "═".repeat(taille_max_quatre + 2));
        let ligne_dessus: String = format!("{ligne_dessus_zero}{ligne_dessus_un}{ligne_dessus_deux}{ligne_dessus_trois}{ligne_dessus_quatre}");
        let ligne_dessous_zero: String = format!("╚{}╩", "═".repeat(taille_max_zero + 2));
        let ligne_dessous_un: String = format!("{}╩", "═".repeat(taille_max_un + 2));
        let ligne_dessous_deux: String = format!("{}╩", "═".repeat(taille_max_deux + 2));
        let ligne_dessous_trois: String = format!("{}╩", "═".repeat(taille_max_trois + 2));
        let ligne_dessous_quatre: String = format!("{}╝", "═".repeat(taille_max_quatre + 2));
        let ligne_dessous: String = format!("{ligne_dessous_zero}{ligne_dessous_un}{ligne_dessous_deux}{ligne_dessous_trois}{ligne_dessous_quatre}");
        println!("{ligne_dessus}");
        println!("{chaine}");
        println!("{ligne_dessous}");
    }
}
