use std::fs::{OpenOptions, File};
use std::io::{self, BufReader, BufRead, Write, BufWriter};
use std::time::Instant;
use std::vec;


#[derive(Debug)]
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
    let updated_data = Data {
        id: 999999, // ID de l'entrée à mettre à jour
        nom: "Dupont".to_string(),
        prenom: "Jean".to_string(),
        mail: "jean.dupont@update.com".to_string(),
        telephone: "0123456789".to_string(),
        age: 31,
    };
    
    // Appel de la fonction pour mettre à jour la donnée
    update_data(updated_data.id, &updated_data)?;
    println!("🕛 insert_data => {:?}", chrono.elapsed());


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
    let file = File::create("database1.txt")?;
    let mut writer = BufWriter::new(file);

    // Boucle sur les données pour les écrire dans le fichier
    for person in data {
        writeln!(writer, "{},{},{},{},{},{}", person.id,person.nom,person.prenom,person.mail,person.telephone,person.age)?;
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