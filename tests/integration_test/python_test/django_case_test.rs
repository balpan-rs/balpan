#[cfg(test)]
mod django_case_test {
    use indoc::indoc;
    use balpan::analyzer::{Analyzer,Traversable};
    use balpan::grammar::{fetch_grammars, build_grammars};

    fn assert_analyzed_source_code(source_code: &str, expected: &str, language: &str) {
        fetch_grammars().unwrap();
        build_grammars(None).unwrap();

        let analyzer = Analyzer {
            source_code: source_code.to_string(),
            language: language.to_string(),
        };

        let writer_queue = &analyzer.analyze();
        let mut string_vector = vec![];

        for line in writer_queue {
            string_vector.push(String::from(line));
        }


        let actual: String = string_vector
            // .iter()
            // .map( |str| { str } )
            // .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_class_definition_within_class() {
        let source_code = indoc! {"
        class Car(models.Model):
            name = models.CharField(max_length=20)
            default_parts = models.ManyToManyField(Part)
            optional_parts = models.ManyToManyField(Part, related_name=\"cars_optional\")

            class Meta:
                ordering = (\"name\",)"};

        let result = indoc! {"
        # [TODO]
        class Car(models.Model):
            name = models.CharField(max_length=20)
            default_parts = models.ManyToManyField(Part)
            optional_parts = models.ManyToManyField(Part, related_name=\"cars_optional\")

            # [TODO]
            class Meta:
                ordering = (\"name\",)"};

        assert_analyzed_source_code(source_code, result, "python")
    }


    #[test]
    fn test_decorated_definitions_within_class_definition() {
        let source_code = indoc! {"      
        class Choices(enum.Enum, metaclass=ChoicesMeta):
            \"\"\"Class for creating enumerated choices.\"\"\"

            @DynamicClassAttribute
            def label(self):
                return self._label_

            @property
            def do_not_call_in_templates(self):
                return True"};

        let result = indoc! {"      
        # [TODO]
        class Choices(enum.Enum, metaclass=ChoicesMeta):
            \"\"\"Class for creating enumerated choices.\"\"\"

            # [TODO]
            @DynamicClassAttribute
            def label(self):
                return self._label_

            # [TODO]
            @property
            def do_not_call_in_templates(self):
                return True"};

        assert_analyzed_source_code(source_code, result, "python")
    }
}
