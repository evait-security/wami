use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct YamlTemplate {
    pub id: String, // A unique name in lower case letters.
    pub title: String, // The real name of the tool / software.
    pub tags: Vec<String>, // An vector of lower case tags for the search function.
    pub description: String, // A longer text describing the too and what it does. 
    pub references: Vec<String>, // Links to websites, repositories, or other resources.
    pub why_not: Vec<String>, // Links or references to other lake ids.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_yaml_template() -> Result<(), serde_yaml::Error> {
        let yaml = r#"
            id: "template1"
            title: "Template 1"
            tags:
              - "tag1"
              - "tag2"
            description: "This is a sample template."
            references:
              - "https://example.com"
            why_not:
              - "template 2"
        "#;

        let template: YamlTemplate = serde_yaml::from_str(yaml)?;
        assert_eq!(template.id, "template1");
        assert_eq!(template.title, "Template 1");
        assert_eq!(template.tags, vec!["tag1", "tag2"]);
        assert_eq!(template.description, "This is a sample template.");
        assert_eq!(template.references, vec!["https://example.com"]);
        assert_eq!(template.why_not, vec!["tango_ola"]);

        Ok(())
    }

    #[test]
    fn test_unique_id() {
        let template1 = YamlTemplate {
            id: "template1".to_owned(),
            title: "Template 1".to_owned(),
            tags: vec![],
            description: "".to_owned(),
            references: vec![],
            why_not: vec![]
        };
        
        let template2 = YamlTemplate {
            id: "template2".to_owned(),
            title: "Template 2".to_owned(),
            tags: vec![],
            description: "".to_owned(),
            references: vec![],
            why_not: vec![]
        };
        
        assert_ne!(template1.id, template2.id);
    }
    
    #[test]
    fn test_title_property() {
        let template = YamlTemplate {
            id: "template1".to_owned(),
            title: "Template 1".to_owned(),
            tags: vec![],
            description: "".to_owned(),
            references: vec![],
            why_not: vec![]
        };
        
        assert_eq!(template.title, "Template 1");
    }
    
    #[test]
    fn test_tags_property() {
        let template = YamlTemplate {
            id: "template1".to_owned(),
            title: "".to_owned(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            description: "".to_owned(),
            references: vec![],
            why_not: vec![]
        };
        
        assert_eq!(template.tags, vec!["tag1", "tag2"]);
    }
     
    #[test]
    fn test_description_property() {
        let template = YamlTemplate {
            id: "template1".to_owned(),
            title: "".to_owned(),
            tags: vec![],
            description: "This is a sample template.".to_owned(),
            references: vec![],
            why_not: vec![]
        };
        
        assert_eq!(template.description, "This is a sample template.");
    }
    
    #[test]
    fn test_references_property() {
        let template = YamlTemplate {
            id: "template1".to_owned(),
            title: "".to_owned(),
            tags: vec![],
            description: "".to_owned(),
            references: vec!["https://example.com".to_owned()],
            why_not: vec![]
        };
        
        assert_eq!(template.references, vec!["https://example.com"]);
    }

    #[test]
    fn test_why_not() {
        let template = YamlTemplate {
            id: "template1".to_owned(),
            title: "".to_owned(),
            tags: vec![],
            description: "".to_owned(),
            references: vec![],
            why_not: vec!["tango_ola".to_owned()]
        };

        assert_eq!(template.why_not, vec!["tango_ola"]);
    }

}