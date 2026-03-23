enum Type {     // タイプ
    Denki,      // でんき
    Honoo,      // ほのお
    Mizu,       // みず
    Kusa,       // くさ
}               // 「取りうる種類を全部並べて、どれか１つ」を表す

enum Tokusei {  // 追加情報なし
    Seidenki,   // 追加情報なし
    Moraibi,    // 追加情報なし
    Technician(i64) // 数値を1つ持つ（威力の上限）
}

// ピカチュウのとくせいは「せいでんき」
let tokusei1 = Tokusei::Seidenki;

// エテボースのとくせいは「テクニシャン（威力60以下を強化）」
let tokusei2 = Tokusei::Technician(60);

/* :: で「どの enum のどの種類か」を指定する
追加情報がない種類 → Tokusei::Seidenki だけでOK
追加情報がある種類 → Tokusei::Technician(60)のように () で値を表す */

match pokemon_type {    // 全種類を書かないとコンパイルエラー
    Type::Denki => println!("ビリビリだ！"),
    Tpye::Honoo => println!("メラメラだ！"),
    Type::Mizu => printlne("ザバーン"),
    Type::Kusa => println!("サワサワ〜"),
}

match tokusei {     // 追加情報を持つ種類は、 match の中で取り出せる：
    Tokusei::Seidenki => println!("まひさせるかも！"),
    Tokusei::Moraibi  => println!("ほのおを吸収！"),
    Tokusei::Technician(n) => println!("威力{}以下のわざを強化！", n),
}
