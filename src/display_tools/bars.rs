pub fn get_bar(value: i32, min: i32, max: i32) -> String {
    // Нормализуем сигнал в диапазоне от 0 до 100
    let normalized_value = if value < min {
        0
    } else if value > max {
        100
    } else {
        ((value - min) * 100 / (max - min))
    };

    // Определяем количество баров на основе нормализованного сигнала
    let bars = match normalized_value {
        s if s >= 100 => 5,
        s if s >= 75 => 4,
        s if s >= 50 => 3,
        s if s >= 25 => 2,
        s if s >= 0 => 1,
        _ => 0,
    };

    // Создаем строку для отображения баров
    let mut bar = String::new();
    for _ in 0..bars {
        bar.push('█');
    }
    for _ in bars..5 {
        bar.push('░');
    }

    bar
}