use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

fn term_fq(word_map: &HashMap<String, f32>) -> HashMap<String, f32> {
    let total_freq_count: f32 = word_map.values().sum();
    let mut tf_table: HashMap<String, f32> = HashMap::new();
    for (word, count) in word_map {
        // let tdf_val: f32 = (count as f32) / (total_freq_count as f32);
        tf_table.insert(word.to_string(), count / total_freq_count);
    }
    tf_table
}

fn inv_doc_fq(
    word_map: &HashMap<String, f32>,
    no_of_docs: &f32,
    word_density: &mut HashMap<String, f32>,
) -> HashMap<String, f32> {
    let mut inf_table: HashMap<String, f32> = HashMap::new();
    let default_map_key: f32 = 0.0f32;
    for word in word_map.keys() {
        let total_occurence = word_density.get(word).unwrap_or(&default_map_key);
        let idf = (no_of_docs / (1.0f32 + total_occurence)).log10();
        inf_table.insert(word.to_string(), idf);
    }
    inf_table
}

fn populate_wordmap(
    reader: BufReader<File>,
    word_map: &mut HashMap<String, f32>,
    default_map_key: &f32,
    _no_of_docs: &f32,
    word_density: &mut HashMap<String, f32>,
) {
    let word_done_map: HashSet<String> = HashSet::new();
    for line in reader.lines() {
        let line_content = line;
        match line_content {
            Ok(line) => {
                let words: Vec<&str> = line.split_whitespace().collect();
                for word in words {
                    // temp variable creation here is important to avoid rustc --explain E0716
                    let ps_word = word.to_lowercase();
                    let word = ps_word.as_str();
                    if !word_map.contains_key(word) {
                        word_map.insert(word.to_string(), 1.0f32);
                    } else {
                        let cur_value = word_map.get(word).unwrap_or(default_map_key);
                        word_map.insert(word.to_string(), cur_value + 1.0f32);
                    }
                    if !word_done_map.contains(word) {
                        if !word_density.contains_key(word) {
                            word_density.insert(word.to_string(), 1.0f32);
                        } else {
                            let cur_value = word_density.get(word).unwrap_or(default_map_key);
                            word_density.insert(word.to_string(), cur_value + 1.0f32);
                        }
                    }
                }
            }
            Err(_) => println!("Error occured while reading the file"),
        }
    }
}

fn read_file(docs: &Vec<String>, no_of_docs: &f32) {
    let mut word_density: HashMap<String, f32> = HashMap::new();
    let default_map_key: f32 = 0.0f32;
    let mut word_map: HashMap<String, f32> = HashMap::new();
    // let filename2 = "data/sample2.txt";
    // let f = File::open(filename);
    for doc in docs.iter() {
        let filename = doc;
        let f = File::open(filename);
        match f {
            Ok(file) => {
                let reader = BufReader::new(file);
                populate_wordmap(
                    reader,
                    &mut word_map,
                    &default_map_key,
                    no_of_docs,
                    &mut word_density,
                );
            }
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    println!("File does not exist");
                } else {
                    println!("Some error occured");
                }
            }
        }
    }

    println!("No. of docs analyzed : {no_of_docs}");

    let _tf: HashMap<String, f32> = term_fq(&word_map);
    let _idf: HashMap<String, f32> = inv_doc_fq(&word_map, no_of_docs, &mut word_density);

    let mut sorted_vec: Vec<(&String, &f32)> = _tf.iter().collect();
    // sorted_vec.sort_by_key(|x| x.0);
    sorted_vec.sort_by(|a, b| b.0.cmp(a.0));
    // println!("Reading TF Now..");
    // for (key, val) in sorted_vec.iter() {
    //     println!("{key} : {val}");
    // }

    println!("----------------------------------------------");
    // println!("Reading IDF Now..");
    let mut sorted_vec2: Vec<(&String, &f32)> = _idf.iter().collect();
    // sorted_vec.sort_by_key(|x| x.0);
    // sorted_vec2.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    sorted_vec2.sort_by(|a, b| b.0.cmp(a.0));

    let mut tf_idf_map: HashMap<String, f32> = HashMap::new();
    for (tf_item, idf_item) in sorted_vec.iter().zip(sorted_vec2.iter()) {
        let (word, _tf_val) = tf_item;
        let (_, _idf_val) = idf_item;
        let tf_idf_val = (*_tf_val) * (*_idf_val);

        if !tf_idf_map.contains_key(&word as &str) {
            tf_idf_map.insert(word.to_string(), tf_idf_val);
        }
        // println!("{word} : {word2}")
    }

    let mut sorted_tfidf_vec: Vec<(&String, &f32)> = tf_idf_map.iter().collect();
    sorted_tfidf_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("Reading TF-IDF Now..");
    for (word, count) in sorted_tfidf_vec {
        println!("{word} : {count}");
    }
}

fn main() {
    let no_of_docs: f32 = 5.0f32;
    let mut docs: Vec<String> = Vec::new();
    for i in 1..5 + 1 {
        let filename = format!("data/sample{i}.txt");
        println!("{filename} added");
        docs.push(filename);
    }
    read_file(&docs, &no_of_docs);
}
