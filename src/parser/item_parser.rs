use super::prelude::*;

pub fn parse_item<'a>(item: Pair<'a, Rule>, pratt: &PrattParser<Rule>) -> Option<Item<'a>> {
    match item.as_rule() {
        Rule::let_stmt => {
            let mut pairs = item.into_inner();
            let ident = Ident(pairs.next().unwrap().as_str());
            let expr = pratt_parse(pairs.next().unwrap().into_inner(), pratt);
            Some(Item::DefItem { ident, expr })
        }
        Rule::expr => {
            let expr = pratt_parse(item.into_inner(), pratt);
            Some(Item::Expr(Box::new(expr)))
        }
        Rule::EOI => None,
        _ => unreachable!(),
    }
}
