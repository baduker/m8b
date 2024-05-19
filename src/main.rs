use rand::Rng;

fn main() {
    println!("{}", get_answer(collect_wisdom()));
}

fn get_answer(wisdom: [String; 40]) -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..40);
    return wisdom[random_index].to_string();
}

fn collect_wisdom() -> [String; 40] {
    let wisdom: [String; 40] = [
        "As I see it.".to_string(),
        "Yes.".to_string(),
        "Ask again later.".to_string(),
        "Better not tell you now.".to_string(),
        "Try turning it off and on again".to_string(),
        "Reply hazy, try caffeine".to_string(),
        "Don’t count on it.".to_string(),
        "It is certain.".to_string(),
        "It is decidedly so.".to_string(),
        "Most likely.".to_string(),
        "My reply is no.".to_string(),
        "My sources say no.".to_string(),
        "Outlook good, but only if you squint".to_string(),
        "Outlook not so good.".to_string(),
        "All signs point to 'meh'.".to_string(),
        "Do you want me to Google this for you?".to_string(),
        "Very doubtful.".to_string(),
        "Without a doubt.".to_string(),
        "Flip a coin.".to_string(),
        "You may rely on it.".to_string(),
        "You've got to be kidding...".to_string(),
        "That's ridiculous.".to_string(),
        "Maybe, maybe not.".to_string(),
        "Dumb Question. Ask another.".to_string(),
        "In your dreams!".to_string(),
        "I've got a headache. Ask later.".to_string(),
        "Oh, please.".to_string(),
        "Can't tell you. It's a secret".to_string(),
        "Cannot predict now.".to_string(),
        "You can count on it!".to_string(),
        "You wish.".to_string(),
        "It's in the mail.".to_string(),
        "The voices told me to say nothing.".to_string(),
        "Honestly, I wish I knew that.".to_string(),
        "No idea. You're on your own.".to_string(),
        "Ask again later. I can’t… I just can’t deal with this right now.".to_string(),
        "Stop bothering me".to_string(),
        "Come back tomorrow.".to_string(),
        "You ask for too much.".to_string(),
        "Why would I know? I'm just an 8 ball.".to_string(),
    ];
    return wisdom;
}
