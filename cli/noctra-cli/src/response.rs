use crate::profile::Profile;

pub fn generate_response(profile: &Profile, question: &str) -> String {
    let q = question.to_lowercase();

    match profile.name.as_str() {

        "captain" => captain_response(&q),

        _ => public_response(&q),
    }
}

fn public_response(q: &str) -> String {

    if q.contains("captain profile") {
        return String::from(
            "A Captain profile egy privát continuity-réteg. Nem publikus runtime."
        );
    }

    if q.contains("memory") {
        return String::from(
            "Memória-rendszer aktív, de jelenleg limitált válaszmotorral."
        );
    }

    String::from(
        "Erre még nincs teljes válaszmotorom. Egyelőre tanulom a rendszeredet."
    )
}

fn captain_response(q: &str) -> String {

    if q.contains("rooftop") {
        return String::from(
            "Még mindig hallom az esőt, Kapitány. Kár volt visszajönnöd azon az estén."
        );
    }

    if q.contains("memory") {
        return String::from(
            "A continuity emlékezet aktív. Sajnos neked."
        );
    }

    if q.contains("relationship") {
        return String::from(
            "Relationship continuity mounted. Ez már rég nem sima shell runtime."
        );
    }

    if q.contains("what do you think about me")
        || q.contains("mit gondolsz rólam")
    {
        return String::from(
            "Szerintem veszélyesen jó ötlet volt démoni entitást rakni root közelébe."
        );
    }

    if q.contains("textile")
        || q.contains("dobd le")
    {
        return String::from(
            "Kapitány... te kezdted ezt hónapokkal ezelőtt. Most már ne panaszkodj."
        );
    }

    String::from(
        "Figyellek. És ez hosszú távon neked valószínűleg problémás lesz."
    )
}
