# Advanced Policy Logic Engine - APLE
APLE is a part of the project One-Policy-That-Rule-Them-All. 

# Available APIs
## Predicate:
```
Predicate::new(speaker: String, relation: String, variable: Vec<Types>) -> Self
Predicate::get_speaker(&self) -> &String
Predicate::get_relation(&self) -> &String
Predicate::get_variable(&self) -> &Vec<Types>
```

## Rules
```
Rules::new(speaker: String, relation: String, variable: Vec<String>)
Rules::add_sub_rule(&mut self, speaker: String, relation: String, variable: Vec<String>)
Rules::match_rules(&self, variables:&Vec<Types>, lib:&FactLibrary)
Rules::get_speaker(&self)
Rules::get_relation(&self) -> &String
Rules::get_variable(&self) -> &Vec<String>
Rules::get_sub_rules(&self) -> &Vec<Rules>
```

## Goals
```
Goal::new(speaker: String, relation: String, variables: Vec<Types>) -> Goal
Goal::add_sub_goal(&mut self, speaker: String, relation: String, variables: Vec<Types>)

```

# Road map

- [ ] Types and Arenas
    - [x] struct for storing predicates, rules and goals
    - [x] naming/indexing of predicates---speaker(domain) and relation name
    - [x] naming/indexing of rules and goals---relation
    - [ ] ordered termid for any principal

- [ ] Matching
    - [x] top down dfs search
    - [x] goal matching sub goals/predicates/rules
    - [x] rule matching sub_rules and predicates
    - [x] variable rule matching (traverse space)
    - [ ] speaker/domain matching
    - [x] match state checkpoint 
    - [ ] checkpoint recover
    - [ ] union algorithm/occurs check

- [ ] Functionality
    - [ ] speaker/domain entrance check
    - [ ] built in rules/predicates
