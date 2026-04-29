// Partie 5 — Envoi d'alertes par email (SMTP simple)
// TODO: utiliser la crate lettre pour envoyer des emails

pub struct AlertConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub from: String,
    pub to: String,
}

pub fn send_email(_config: &AlertConfig, _subject: &str, _body: &str) -> anyhow::Result<()> {
    todo!("Implémenter l'envoi d'email via lettre")
}

pub fn log_to_file(_path: &str, _message: &str) -> anyhow::Result<()> {
    todo!("Implémenter l'écriture dans le fichier de log")
}
