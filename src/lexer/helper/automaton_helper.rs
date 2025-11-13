// use std::collections::HashMap;
//
//
// pub fn info_automata(lines: &Vec<String>) -> Result<Automata, Box<dyn Error>> {
//     let alfabeto_input: Vec<String> = lines[0].split(',').map(|s| s.trim().to_string()).collect();
//     let mut alfabeto_exp: Vec<String> = Vec::new();
//
//     for simbolo in alfabeto_input {
//         match simbolo.as_str() {
//             "$" => alfabeto_exp.extend((0..=9).map(|d| d.to_string())), // Dígitos
//             "%" => alfabeto_exp.extend(
//                 // Letras
//                 ('a'..='z').chain('A'..='Z').map(|c| c.to_string()),
//             ),
//             "&" => alfabeto_exp.extend(
//                 // Dígitos y Letras
//                 (0..=9)
//                     .map(|d| d.to_string())
//                     .chain(('a'..='z').chain('A'..='Z').map(|c| c.to_string())),
//             ),
//             _ => alfabeto_exp.push(simbolo.clone()), // Símbolo literal
//         }
//     }
//
//     alfabeto_exp.sort();
//     alfabeto_exp.dedup();
//     let alfabeto = alfabeto_exp;
//
//     let mut char_to_col_index = HashMap::new();
//     for (idx, s) in alfabeto.iter().enumerate() {
//         if let Some(c) = s.chars().next() {
//             char_to_col_index.insert(c, idx);
//         }
//     }
//
//     let estados: Vec<String> = lines[1].split(',').map(|s| s.trim().to_string()).collect();
//
//     let sta_inicial_vec: Vec<String> = lines[2].split(',').map(|s| s.trim().to_string()).collect();
//     let sta_inicial = sta_inicial_vec
//         .get(0)
//         .ok_or("No se especificó un estado inicial")?
//         .clone();
//
//     let sta_final: Vec<String> = lines[3].split(',').map(|s| s.trim().to_string()).collect();
//
//     let num_alfabeto = alfabeto.len();
//     let num_estados = estados.len();
//     let mut matrix: Vec<Vec<String>> = vec![vec!["".to_string(); num_alfabeto]; num_estados];
//
//     for line in &lines[4..] {
//         if line.is_empty() {
//             continue;
//         };
//
//         // Formato esperado: EstadoOrigen,Consumible=EstadoDestino
//         let partes: Vec<&str> = line
//             .split(|c| c == ',' || c == '=')
//             .map(|s| s.trim())
//             .collect();
//
//         if partes.len() != 3 {
//             return Err(format!("Formato de transicion invalido en linea: '{}'", line).into());
//         }
//
//         let estado_origen = partes[0];
//         let consumable = partes[1];
//         let estado_destino = partes[2];
//
//         let origen_idx = estados
//             .iter()
//             .position(|s| s == estado_origen)
//             .ok_or_else(|| format!("Error: estado de origen '{}' no encontrado.", estado_origen))?;
//
//         let consumable_idx = alfabeto
//             .iter()
//             .position(|s| s == consumable)
//             .ok_or_else(|| {
//                 format!(
//                     "Error: simbolo '{}' no existe en el alfabeto expandido",
//                     consumable
//                 )
//             })?;
//
//         if !matrix[origen_idx][consumable_idx].is_empty() {
//             return Err(format!(
//                 "Error: Ya existe una transicion para ({}, {}). Esto sugiere que no es un DFA.",
//                 estado_origen, consumable
//             )
//             .into());
//         }
//
//         matrix[origen_idx][consumable_idx] = estado_destino.to_string();
//     }
//
//     println!("--- Automata Cargado Exitosamente ---");
//     println!("Alfabeto ({}): {:?}", alfabeto.len(), alfabeto);
//     println!("Estados ({}): {:?}", estados.len(), estados);
//     println!("Estado Inicial: {}", sta_inicial);
//     println!("Estados Finales: {:?}", sta_final);
//
//
//     Ok(Automata {
//         alfabeto,
//         estados,
//         sta_inicial,
//         sta_final,
//         matrix,
//         char_to_col_index,
//     })
// } 
