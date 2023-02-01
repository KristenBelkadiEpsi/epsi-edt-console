use chrono::{DateTime, Datelike, Duration, TimeZone, Utc, Weekday};
use scraper::{ElementRef, Html, Selector};
use std::env;
use std::str::FromStr;

fn weekday_en_jour(weekday: &Weekday) -> String {
    match weekday {
        Weekday::Mon => "lundi".to_string(),
        Weekday::Tue => "mardi".to_string(),
        Weekday::Wed => "mercredi".to_string(),
        Weekday::Thu => "jeudi".to_string(),
        Weekday::Fri => "vendredi".to_string(),
        Weekday::Sat => "samedi".to_string(),
        Weekday::Sun => "dimanche".to_string(),
    }
}

async fn recuperer_infos(
    identifiant: &String,
    date: &DateTime<Utc>,
) -> (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
) {
    let date_chaine: String = date.format("%m/%d/%Y").to_string();
    let url: &String = &format!("https://edtmobiliteng.wigorservices.net//WebPsDyn.aspx?action=posEDTBEECOME&serverid=C&Tel={}&date={}", identifiant, date_chaine);
    let requete = reqwest::get(url).await.unwrap();

    if requete.status().is_success() {
        let texte_html: String = requete.text().await.unwrap();
        let parseur_html: Html = Html::parse_document(&*texte_html);
        let mut tableau_noms: Vec<String> = vec![];
        let mut tableau_profs: Vec<String> = vec![];
        let mut tableau_horaires: Vec<String> = vec![];
        let mut tableau_salles: Vec<String> = vec![];
        let mut tableau_jours: Vec<String> = vec![];
        let mut jours_semaine: Vec<DateTime<Utc>> = vec![];
        let mut tailles: Vec<f64> = vec![];

        for i in 1..=7 {
            let lundi_date: DateTime<Utc> =
                *date - Duration::days(date.weekday().num_days_from_monday() as i64 + 1);
            jours_semaine.push(lundi_date + Duration::days(i as i64));
        }
        let selecteur_case: Selector = Selector::parse(".Case").unwrap();
        let liste_case: Vec<ElementRef> = parseur_html.select(&selecteur_case).collect();
        liste_case[0..(liste_case.len() - 1)]
            .iter()
            .for_each(|e: &ElementRef| {
                let left_chaine: &str = e
                    .value()
                    .attr("style")
                    .unwrap()
                    .split(";")
                    .collect::<Vec<&str>>()[3];
                let chaine_taille: &str = &left_chaine[5..(left_chaine.len() - 1)];
                let nombre: f64 = f64::from_str(chaine_taille).unwrap();
                tailles.push(nombre);
            });
        tailles.sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap());
        let mut i: usize = 0;
        let mut comp: f64 = tailles[0];
        tailles.iter().for_each(|element: &f64| {
            if *element != comp {
                comp = *element;
                i = i + 1;
            }
            tableau_jours.push(weekday_en_jour(&(jours_semaine[i].weekday())));
        });
        let liste_noms_elements: Vec<ElementRef> = parseur_html
            .select(&Selector::parse(".Case .innerCase .BackGroundCase .TCase .TCase").unwrap())
            .collect::<Vec<ElementRef>>();
        liste_noms_elements.iter().for_each(|e: &ElementRef| {
            tableau_noms.push(e.text().fold("".to_string(), |acc: String, element: &str| {
                format!("{}{}", acc, element)
            }))
        });
        let liste_profs_elements: Vec<ElementRef> = parseur_html
            .select(&Selector::parse(".Case .innerCase .BackGroundCase .TCase .TCProf").unwrap())
            .collect::<Vec<ElementRef>>();
        liste_profs_elements.iter().for_each(|e: &ElementRef| {
            tableau_profs.push(e.text().fold("".to_string(), |acc: String, element: &str| {
                format!("{}{}", acc, element)
            }))
        });
        let liste_horaires_elements: Vec<ElementRef> = parseur_html
            .select(&Selector::parse(".Case .innerCase .BackGroundCase .TCase .TChdeb").unwrap())
            .collect::<Vec<ElementRef>>();
        liste_horaires_elements.iter().for_each(|e: &ElementRef| {
            tableau_horaires.push(e.text().fold("".to_string(), |acc: String, element: &str| {
                format!("{}{}", acc, element)
            }))
        });
        let liste_salles_elements: Vec<ElementRef> = parseur_html
            .select(&Selector::parse(".Case .innerCase .BackGroundCase .TCase .TCSalle").unwrap())
            .collect::<Vec<ElementRef>>();
        liste_salles_elements.iter().for_each(|e: &ElementRef| {
            tableau_salles.push(e.text().fold("".to_string(), |acc: String, element: &str| {
                format!("{}{}", acc, element)
            }))
        });
        (
            tableau_jours,
            tableau_noms,
            tableau_profs,
            tableau_horaires,
            tableau_salles,
        )
    } else {
        panic!("erreur !");
    }
}

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
        let ligne_dessus:String = format!("{ligne_dessus_zero}{ligne_dessus_un}{ligne_dessus_deux}{ligne_dessus_trois}{ligne_dessus_quatre}");
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
