use std::fs::{OpenOptions, File};
use std::io::{self, BufReader, BufRead, Write, BufWriter};
use std::time::Instant;
use std::vec;
use serde;  // Nécessaire pour la sérialisation
use bincode;  // Pour la sérialisation binaire


#[derive(Debug, serde::Serialize)]
struct Data {
    id: isize,
    nom: String,
    prenom: String,
    mail: String,
    telephone: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let chrono: Instant = Instant::now();
    ecrit_data()?;
    println!("🕛 ecrit_data => {:?}", chrono.elapsed());

    let chrono: Instant = Instant::now();
    ecrit_data_2()?;
    println!("🕛 ecrit_data_2 => {:?}", chrono.elapsed());


    Ok(())
}

fn _lit_data() -> Result<(), Box<dyn std::error::Error>>  {
    // Ouvre le fichier en mode lecture
    let file = File::open("database1.txt")?;
    
    // Utilise BufReader pour optimiser la lecture
    let reader = BufReader::new(file);

    let mut data_list: Vec<Data> = Vec::new(); // Vecteur pour stocker les données

    // Parcourt le fichier ligne par ligne
    for line in reader.lines() {
        let line = line?; // Gestion des erreurs pour chaque ligne lue
        
        // Sépare la ligne par des virgules
        let fields: Vec<&str> = line.split(',').collect();

        // Vérifie que nous avons le bon nombre de champs
        if fields.len() == 6 {
            // Crée une instance de Data en convertissant les champs au bon type
            let data = Data {
                id: fields[0].trim().parse()?, // Convertit en isize
                nom: fields[1].trim().to_string(), // String
                prenom: fields[2].trim().to_string(), // String
                mail: fields[3].trim().to_string(), // String
                telephone: fields[4].trim().to_string(), // String
                age: fields[5].trim().parse()?, // Convertit en u32
            };
            data_list.push(data); // Ajoute l'instance au vecteur
        } else {
            eprintln!("Ligne mal formatée : {}", line); // Avertit si la ligne est mal formatée
        }
    }

    // test select
    for data in data_list {
        if data.id == 524547 {
            println!("{data:?}");
            break;
        }
    }

    Ok(())
}

fn ecrit_data() -> Result<(), Box<dyn std::error::Error>> {
    let NBR_DATA = 1_000_000;
    let mut data: Vec<Data> = Vec::new();

    for i in 0..NBR_DATA {
        let id: isize = i;
        let nom: String = format!("nom{}", i);
        let prenom: String = format!("prenom{}", i);
        let mail: String = format!("mail{}", i);
        let telephone: String = format!("telephone{}", i);
        let age: u32 = (i % 100).try_into().unwrap();
        data.push(Data {id,nom,prenom,mail,telephone,age})
    }

    // Ouvre ou crée un fichier pour écrire
    let file_id = File::create("test_1/id.txt")?;
    let file_nom = File::create("test_1/nom.txt")?;
    let file_prenom = File::create("test_1/prenom.txt")?;
    let file_mail = File::create("test_1/mail.txt")?;
    let file_telephone = File::create("test_1/telephone.txt")?;
    let file_age = File::create("test_1/age.txt")?;

    let mut writer_id = BufWriter::new(file_id);
    let mut writer_nom = BufWriter::new(file_nom);
    let mut writer_prenom = BufWriter::new(file_prenom);
    let mut writer_mail = BufWriter::new(file_mail);
    let mut writer_telephone = BufWriter::new(file_telephone);
    let mut writer_age = BufWriter::new(file_age);

    // Boucle sur les données pour les écrire dans le fichier
    for person in data {
        writeln!(writer_id, "{}", person.id)?;
        writeln!(writer_nom, "{}", person.nom)?;
        writeln!(writer_prenom, "{}", person.prenom)?;
        writeln!(writer_mail, "{}", person.mail)?;
        writeln!(writer_telephone, "{}", person.telephone)?;
        writeln!(writer_age, "{}", person.age)?;
    }

    Ok(())
}

fn ecrit_data_2() -> Result<(), Box<dyn std::error::Error>> {
    const NBR_DATA: usize = 1_000_000;
    let mut data: Vec<Data> = Vec::with_capacity(NBR_DATA);

    // Génération des données
    for i in 0..NBR_DATA {
        let id = i as isize;
        let nom = format!("nom{}", i);
        let prenom = format!("prenom{}", i);
        let mail = format!("mail{}", i);
        let telephone = format!("telephone{}", i);
        let age = (i % 100) as u32;
        data.push(Data { id, nom, prenom, mail, telephone, age });
    }

    // Ouvre ou crée un fichier binaire pour écrire
    let file_id = File::create("test_2/id.bin")?;
    let file_nom = File::create("test_2/nom.bin")?;
    let file_prenom = File::create("test_2/prenom.bin")?;
    let file_mail = File::create("test_2/mail.bin")?;
    let file_telephone = File::create("test_2/telephone.bin")?;
    let file_age = File::create("test_2/age.bin")?;

    let mut writer_id = BufWriter::new(file_id);
    let mut writer_nom = BufWriter::new(file_nom);
    let mut writer_prenom = BufWriter::new(file_prenom);
    let mut writer_mail = BufWriter::new(file_mail);
    let mut writer_telephone = BufWriter::new(file_telephone);
    let mut writer_age = BufWriter::new(file_age);

    // Sérialisation et écriture binaire des données
    for person in &data {
        // Sérialise chaque `person` en binaire et l'écrit dans le fichier
        bincode::serialize_into(&mut writer_id, &person.id)?;
        bincode::serialize_into(&mut writer_nom, &person.nom)?;
        bincode::serialize_into(&mut writer_prenom, &person.prenom)?;
        bincode::serialize_into(&mut writer_mail, &person.mail)?;
        bincode::serialize_into(&mut writer_telephone, &person.telephone)?;
        bincode::serialize_into(&mut writer_age, &person.age)?;
    }

    Ok(())
}


fn insert_data() -> Result<(), Box<dyn std::error::Error>> {
    // Ouvre le fichier en mode ajout
    let file = OpenOptions::new()
        .append(true) // Ouvrir en mode ajout
        .create(true) // Créer le fichier s'il n'existe pas
        .open("database1.txt")?; // Spécifie le chemin du fichier

    let id: isize = 1;
    let nom: String = format!("nom{}", 1);
    let prenom: String = format!("prenom{}", 1);
    let mail: String = format!("mail{}", 1);
    let telephone: String = format!("telephone{}", 1);
    let age: u32 = (50 % 100).try_into().unwrap();
    let new_data = Data {id,nom,prenom,mail,telephone,age};

    // Utilise un BufferWriter pour optimiser l'écriture
    let mut writer = BufWriter::new(file);
    
    // Écrit la nouvelle ligne dans le fichier
    writeln!(writer, "{},{},{},{},{},{}", new_data.id, new_data.nom, new_data.prenom, new_data.mail, new_data.telephone, new_data.age)?;
    
    Ok(())
}


fn update_data(id_to_update: isize, updated_data: &Data) -> Result<(), Box<dyn std::error::Error>> {
    // Ouvre le fichier en mode lecture
    let file = File::open("database1.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new(); // Vecteur pour stocker toutes les lignes

    // Parcourt le fichier ligne par ligne
    for line in reader.lines() {
        let line = line?; // Gestion des erreurs pour chaque ligne lue
        
        // Sépare la ligne par des virgules
        let fields: Vec<&str> = line.split(',').collect();

        // Vérifie que nous avons le bon nombre de champs
        if fields.len() == 6 {
            // Vérifie si l'ID correspond à l'ID que l'on veut mettre à jour
            if fields[0].trim().parse::<isize>()? == id_to_update {
                // Met à jour la ligne avec les nouvelles données
                lines.push(format!("{},{},{},{},{},{}", 
                    updated_data.id, 
                    updated_data.nom, 
                    updated_data.prenom, 
                    updated_data.mail, 
                    updated_data.telephone, 
                    updated_data.age));
            } else {
                lines.push(line); // Garde la ligne existante
            }
        } else {
            eprintln!("Ligne mal formatée : {}", line); // Avertit si la ligne est mal formatée
            lines.push(line); // Garde la ligne existante
        }
    }

    // Ouvre le fichier en mode écriture (écrase l'ancien contenu)
    let file = OpenOptions::new()
        .write(true)
        .truncate(true) // Écrase le contenu du fichier
        .open("database1.txt")?;
    
    let mut writer = BufWriter::new(file);

    // Écrit toutes les lignes mises à jour dans le fichier
    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}