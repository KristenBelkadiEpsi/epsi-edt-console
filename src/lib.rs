use chrono::{DateTime, Datelike, Duration, Utc, Weekday};
use scraper::{ElementRef, Html, Selector};
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

pub async fn recuperer_infos(
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
        let parseur_html: Html = Html::parse_document(&texte_html);
        let selecteur_pas_de_cours: Selector = Selector::parse("div.Case:nth-child(47)").unwrap();
        if parseur_html.select(&selecteur_pas_de_cours).count() != 0 {
            (
                vec![
                    "lundi".to_string(),
                    "mardi".to_string(),
                    "mercredi".to_string(),
                    "jeudi".to_string(),
                    "vendredi".to_string(),
                ],
                vec!["".to_string(); 5],
                vec!["".to_string(); 5],
                vec!["".to_string(); 5],
                vec!["".to_string(); 5],
            )
        } else {
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
            std::thread::sleep(std::time::Duration::from_secs(1));
            let liste_case: Vec<ElementRef> = parseur_html.select(&selecteur_case).collect();
            liste_case[0..(liste_case.len() - 1)]
                .iter()
                .for_each(|e: &ElementRef| {
                    let left_chaine: &str = e
                        .value()
                        .attr("style")
                        .unwrap()
                        .split(';')
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
                    i += 1;
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
                .select(
                    &Selector::parse(".Case .innerCase .BackGroundCase .TCase .TCProf").unwrap(),
                )
                .collect::<Vec<ElementRef>>();
            liste_profs_elements.iter().for_each(|e: &ElementRef| {
                tableau_profs.push(e.text().fold("".to_string(), |acc: String, element: &str| {
                    format!("{}{}", acc, element)
                }))
            });
            let liste_horaires_elements: Vec<ElementRef> = parseur_html
                .select(
                    &Selector::parse(".Case .innerCase .BackGroundCase .TCase .TChdeb").unwrap(),
                )
                .collect::<Vec<ElementRef>>();
            liste_horaires_elements.iter().for_each(|e: &ElementRef| {
                tableau_horaires.push(
                    e.text().fold("".to_string(), |acc: String, element: &str| {
                        format!("{}{}", acc, element)
                    }),
                )
            });
            let liste_salles_elements: Vec<ElementRef> = parseur_html
                .select(
                    &Selector::parse(".Case .innerCase .BackGroundCase .TCase .TCSalle").unwrap(),
                )
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
        }
    } else {
        panic!("erreur !");
    }
}
