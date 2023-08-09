# Advanced Policy Logic Engine - APLE
APLE is a part of the project One-Policy-That-Rule-Them-All. APLE presents a new logic engine in a top-down matching manner (similar to Prolog). The implementation of the logic engine follows the language concept of [Warren Abstract Machine Book](https://github.com/mthom/scryer-prolog/blob/master/wambook/wambook.pdf).

This project follows and leverages the concepts and designs from existing rust logic language implementation such as [Datafrog](https://github.com/rust-lang/datafrog), [scryer-prolog](https://github.com/mthom/scryer-prolog) and [logru](https://docs.rs/logru/latest/logru/).

---

## APLE 
APLE is both a subset and an externsion of Prolog. It adopts a similar Top-Down matching strategy as the Prolog, which starts with the goal to search for compliant predicates that fulfill the goals. This strategy perfectly fits our need for supply chain security policy enforcement, since the trust metadata often stores in a remote repository or a different infrastructure.

APLE uses a unification algorithm, which solves equations and matching between symbotic expressions. 

APLE only keeps the core functionalities of a full size Prolog engine and extends it with three features:

- fetch external data on query
- signature validation in the compiler
- introducing the speaker of the predicates

## APLE syntax
APLE shares a similar syntax as tranditional Prolog language. Both rules and queries in APLE are logic predicates in the form of a speaker states a relation about a set of facts. As shown in the below example, an APLE predicate shares a similar form as a programing language function, where the relation looks like a function name and the speaker and facts are separated by `,` within `(` and `)`. The period `.` makes the end of the predicates.
```
relation(speaker, fact1,...,factn).
```
APLE also supports defining complex compositions of subrequirements for rules. In APLE, the main rule is ended by `:-` and the main_rule serves as the top level goal. In order to fulfill the main_rule, each sub_rule needs to fulfill itself separately. The sub_rules are separated by `,` and ended by `.`.
```
main_rule(speaker, fact1,...,factn):-
    sub_rule1(speaker, arg1,...,argn),
    sub_rule2(speaker, farg1,...,fargn).
```
APLE also allows variable expressions for facts or speakers to match any compliant predicates or principals. APLE marks a variable arguments with `$` in front of the name. For example, the below example, `$speaker` and `$fact2` are variables and will match any term that fulfills this rule. `fact1` is a strict string term, which means these rules need to contain string `fact1` in order to fulfill themselves.
```
main_rule($speaker, fact1):-
    sub_rule1($speaker, fact1, $fact2),
    sub_rule2($speaker, fact1, $fact2).
```



## Road map

- [x] Types and Arenas
    - [x] syms, principals, predicates, terms
    - [x] rules and goals
    - [x] available types for logic facts - String/Number/Array
    - [x] naming/indexing of predicates/goals/rules/syms
    - [ ] labels for symbol for extra features
    

- [x] Matching
    - [x] unification algorithm for unifying goal terms and rule terms
    - [x] top down dfs search
    - [x] goal matching sub goals/predicates/rules
    - [x] rule matching sub_rules and predicates
    - [x] variable term matching (traverse space)
    - [x] variable term mapping occurs check 
    - [x] match state checkpoint 
    - [x] checkpoint recover
    - [x] built in rules/predicates
    - [ ] variable name rule matching
    - [ ] speaker/domain/label matching

- [x] Compiler
    - [x] external file fetching
    - [x] compile Tuf metadata
    - [x] compile Intoto metadata
    - [x] compile rules and policies
    - [x] signature validation
    - [x] file hash
    - [x] query on compile back and forth with engine
    - [ ] compile sigstore metadata
    - [ ] yaml support

