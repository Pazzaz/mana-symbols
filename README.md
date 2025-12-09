<div align="center">

# Mana Symbols

<span class="mana_symbols"><img style="height: 1.5em; width: 1.7em; vertical-align: middle" alt="{5}" title="5 generic mana" src="snapshots/mana__5.snap.svg"><img style="height: 1.5em; width: 1.7em; vertical-align: middle" alt="{C}" title="Colorless mana" src="snapshots/mana__c.snap.svg"><img style="height: 1.5em; width: 1.7em; vertical-align: middle" alt="{U}" title="Blue mana" src="snapshots/mana__u.snap.svg"><img style="height: 1.5em; width: 1.7em; vertical-align: middle" alt="{R/G/P}" title="Phyrexian hybrid mana: red or green" src="snapshots/mana__r_g_p.snap.svg"><img style="height: 1.5em; width: 1.7em; vertical-align: middle" alt="{S}" title="Snow mana" src="snapshots/mana__s.snap.svg"></span>

</div>

This is a Rust crate to model [Mana costs][mw:mc] from [Magic the Gathering][wp:mtg] cards. It can parse text representations of mana (e.g. `{5}{U}{U/B}`), sort mana costs and calculate [mana values][mw:mv].

## Supported mana

The types of mana supported by this library are:
- [Generic mana][mw:gm]
- [Colorless mana][mw:clm]
- [Colored][mw:c] mana (including [phyrexian][mw:pm])
- [Hybrid mana][mw:hm] (including generic, colorless and phyrexian)
- [Snow mana][mw:sc]

## Sorting
When sorting mana symbols in mana costs, this library uses an algorithm proposed by [`/u/Mean-Government1436`][reddit:user] in [a post on /r/custommagic][reddit:post]. The developers of Magic the Gathering, [Wizards of the Coast][wp:wotc], have not given any official algorithm for sorting of mana symbols.


<small>

```
Graphical images used in this library, including mana symbols, is copyright Wizards of the Coast, LLC. This library is not produced by or endorsed by Wizards of the Coast.
```

</small>

[mw:mc]:  https://mtg.wiki/page/Mana_cost
[mw:mv]:  https://mtg.wiki/page/Mana_value
[mw:gm]:  https://mtg.wiki/page/Generic_mana
[mw:clm]: https://mtg.wiki/page/Colorless#Colorless_mana
[mw:pm]:  https://mtg.wiki/page/Phyrexian_mana
[mw:c]:   https://mtg.wiki/page/Color
[mw:hm]:  https://mtg.wiki/page/Hybrid_mana
[mw:sc]:  https://mtg.wiki/page/Snow#Snow_costs

[wp:mtg]:  https://en.wikipedia.org/wiki/Magic:_The_Gathering
[wp:wotc]: https://en.wikipedia.org/wiki/Wizards_of_the_Coast

[reddit:user]: https://www.reddit.com/user/Mean-Government1436
[reddit:post]: https://www.reddit.com/r/custommagic/comments/1nhtr3w/guide_for_formatting_mana_costs/