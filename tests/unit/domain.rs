
#[test]
fn check_struct_integrity() {
    
    use mail_tm::structs;

    // And also explicitly check the naming
    let user: structs::User = structs::User::default();
    {
        // Asserting our user by explicitly typed values
        // So we dont have to worry about wrong typing in future
        let password_field: String = String::default();
        let token_field: String = String::default();
        let email_field: String = String::default();
        let letters_field: Vec<structs::LetterShort> = vec![];

        assert!(user.password == password_field, "Password field changed!");
        assert!(user.token == token_field, "Token field changed!");
        assert!(user.email == email_field, "Email field changed!");
        assert!(user.letters == letters_field, "Letters field changed!");
    }

    let addresant: structs::Addresant = structs::Addresant::default();
    
    {
        let name: String = String::default();
        let address: String = String::default();
        
        assert!(addresant.name == name, "Name field of addresant changed!");
        assert!(addresant.address == address, "Address field of addresand changed!");
    }

    let attachment: structs::Attachment = structs::Attachment::default();

    {
        let filename: String = String::default();
        let download_url: String = String::default();
        
        assert!(attachment.filename == filename, "Filename field of attachment changed!");
        assert!(attachment.download_url == download_url, "Address field of attachment changed!");
    }

    let letter: structs::Letter = structs::Letter::default();

    {
        let html: String = String::default();
        let text: String = String::default();
        let download_url: String = String::default();
        
        assert!(letter.download_url == download_url, "Download_url field of letter changed!");
        assert!(letter.html == html, "Html field of letter changed!");
        assert!(letter.text == text, "Text field of letter changed!");
    }
}
