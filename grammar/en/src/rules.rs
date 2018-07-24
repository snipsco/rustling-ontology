use std::f32;

use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain, PeriodComp, Period};

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        b.reg(r"(?:%|p\.c\.|per ?cents?)")?,
        |number, _| Ok(PercentageValue(number.value().value()))
    );
    Ok(())
}

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"sec(?:ond)?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"min(?:ute)?s?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"h(?:(?:(?:ou)?rs?)|r)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"days?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"weeks?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"months?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"years?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1_terminal("quarter of an hour",
                      b.reg(r#"1/4\s?h(?:our)?|(?:a\s)?quarter(?: of an |-)hour"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("half an hour",
                      b.reg(r#"1/2\s?h(?:our)?|half an? hour|an? half hour"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("three-quarters of an hour",
                      b.reg(r#"3/4\s?h(?:our)?|three(?:\s|-)quarters of an hour"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_1_terminal("fortnight",
                      b.reg(r#"(?:a|one)? fortnight"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::days(14).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_3("<integer> more <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"more|less"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2_terminal("number.number hours",
                      b.reg(r#"(\d+)\.(\d+)"#)?,
                      b.reg(r#"hours?"#)?,
                      |text_match, _| {
                          Ok(DurationValue::new(
                              PeriodComp::minutes(
                                  helpers::decimal_hour_in_minute(text_match.group(1), text_match.group(2))?
                              ).into()
                          )
                          )
                      }
    );
    b.rule_2("<integer> and a half hours",
             integer_check_by_range!(0),
             b.reg(r#"and (?:an? )?half hours?"#)?,
             |integer, _| Ok(DurationValue::new(PeriodComp::minutes(integer.value().value * 60 + 30).into()))
    );
    b.rule_3("<integer> <unit-of-duration> and a half",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"and (?:an? )?half"#)?,
             |integer, uod, _| {
                let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
            }
    );
    b.rule_3("<integer> and a half <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"and (?:an? )?half"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| {
                let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
            }
    );
    b.rule_3("<number> h <number>",
             integer_check_by_range!(0),
             b.reg(r#"h(?:ours?)?"#)?,
             integer_check_by_range!(0,59),
             |hour, _, minute| {
                 let hour_period = Period::from(PeriodComp::new(Grain::Hour, hour.value().clone().value));
                 let minute_period = Period::from(PeriodComp::new(Grain::Minute, minute.value().clone().value));
                 Ok(DurationValue::new(hour_period + minute_period))
             }
    );
    b.rule_2("a <unit-of-duration>",
             b.reg(r#"an?"#)?,
             unit_of_duration_check!(),
             |_, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, 1).into()))
    );
    b.rule_2("in <duration>",
             b.reg(r#"in"#)?,
             duration_check!(),
             |_, duration| duration.value().in_present()
    );
    b.rule_3("in <duration>",
             b.reg(r#"in"#)?,
             duration_check!(),
             b.reg(r#"(?:' )? times?"#)?,
             |_, duration, _| duration.value().in_present()
    );
    b.rule_2("for <duration>",
             b.reg(r#"for"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("during <duration>",
             b.reg(r#"during"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("after <duration>",
             b.reg(r#"after"#)?,
             duration_check!(),
             |_, duration| Ok(duration
                 .value()
                 .in_present()?
                 .mark_after_start())
    );
    b.rule_3("<duration> and <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             b.reg(r#"and"#)?,
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, _, b| Ok(a.value() + b.value())
    );

    b.rule_2("<duration> <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, b| Ok(a.value() + b.value())
    );

    b.rule_2("<duration> from now", // "10 minutes from now"
             duration_check!(),
             b.reg(r#"from (?:today|now)"#)?,
             |a, _| a.value().in_present()
    );

    b.rule_3("for <duration> from now", // "for 10 minutes from now"
             b.reg(r#"for"#)?,
             duration_check!(),
             b.reg(r#"from (?:today|now)"#)?,
             |_, duration, _| {
                 let start = helpers::cycle_nth(Grain::Second, 0)?;
                 let end = duration.value().in_present()?;
                 start.span_to(&end, false)
             }
    );

    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"ago"#)?,
             |a, _| a.value().ago()
    );

    b.rule_2("<duration> hence",
             duration_check!(),
             b.reg(r#"hence"#)?,
             |a, _| a.value().in_present()
    );

    b.rule_3("<duration> after <time>",
             duration_check!(),
             b.reg(r#"after"#)?,
             time_check!(),
             |duration, _, time| duration.value().after(time.value())
    );

    b.rule_3("<duration> before <time>",
             duration_check!(),
             b.reg(r#"before"#)?,
             time_check!(),
             |duration, _, time| duration.value().before(time.value())
    );

    b.rule_2("about <duration>",
             b.reg(r#"(?:about|around|approximately|roughly)"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );

    b.rule_2("exactly <duration>",
             b.reg(r#"exactly|precisely"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );

    Ok(())
}

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"seconds?"#)?,
                      |_| CycleValue::new(Grain::Second)
    );

    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"minutes?"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );

    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"h(?:ou)?rs?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );

    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"days?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );

    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"weeks?"#)?,
                      |_| CycleValue::new(Grain::Week)
    );

    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"months?"#)?,
                      |_| CycleValue::new(Grain::Month)
    );

    b.rule_1_terminal("quarter (cycle)",
                      b.reg(r#"(?:quarter|qtr)s?"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );

    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"y(?:ea)?rs?"#)?,
                      |_| CycleValue::new(Grain::Year)
    );

    b.rule_2("this <cycle>",
             b.reg(r#"this|current|coming"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 0)
    );

    b.rule_2("last <cycle>",
             b.reg(r#"last|past|previous"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, -1)
    );

    b.rule_2("next <cycle>",
             b.reg(r#"next|the following"#)?,
             cycle_check!(),
             |_, a| helpers::cycle_nth(a.value().grain, 1)
    );

    b.rule_4("the <cycle> after <time>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             b.reg(r#"after"#)?,
             time_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );

    b.rule_3("<cycle> after <time>",
             cycle_check!(),
             b.reg(r#"after"#)?,
             time_check!(),
             |cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );

    b.rule_4("<cycle> before <time>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             b.reg(r#"before"#)?,
             time_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );

    b.rule_3("<cycle> before <time>",
             cycle_check!(),
             b.reg(r#"before"#)?,
             time_check!(),
             |cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );

    b.rule_3("last n <cycle>",
             b.reg(r#"last|past"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );

    b.rule_3("next n <cycle>",
             b.reg(r#"next"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );

    b.rule_4("<ordinal> <cycle> of <time>",
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"of|in|from"#)?,
             time_check!(),
             |ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );

    b.rule_5("the <ordinal> <cycle> of <time>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"of|in|from"#)?,
             time_check!(),
             |_, ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );

    b.rule_4("the <cycle> of <time>",
             b.reg(r#"the"#)?,
             cycle_check!(),
             b.reg(r#"of"#)?,
             time_check!(),
             |_, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, 0, time.value())
    );

    b.rule_4("<ordinal> <cycle> after <time>",
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"after"#)?,
             time_check!(),
             |ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );

    b.rule_5("the <ordinal> <cycle> after <time>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             cycle_check!(),
             b.reg(r#"after"#)?,
             time_check!(),
             |_, ordinal, cycle, _, time| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, time.value())
    );
    b.rule_2(
        "<ordinal> quarter",
        ordinal_check!(),
        cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
        |ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_3("the <ordinal> quarter",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |_, ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_3("<ordinal> quarter <year>",
             ordinal_check!(),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             time_check!(),
             |ordinal, _, time| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, time.value())
    );
    Ok(())
}


pub fn rules_time(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect <time>",
             time_check!(|time: &TimeValue| !time.latent),
             time_check!(|time: &TimeValue| !time.latent),
             |a, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by \"of\", \"from\", \"'s\"",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#"of|from|for|'s"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_3("intersect by \",\"",
             time_check!(|time: &TimeValue| !time.latent),
             b.reg(r#","#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    b.rule_2("on|in <date>",
             b.reg(r#"[oi]n"#)?,
             time_check!(),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("for <date>",
             b.reg(r#"for"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("for <date>",
             b.reg(r#"for"#)?,
             time_check!(form!(Form::Meal)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("on a <named-day>",
             b.reg(r#"on a"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, a| Ok(a.value().clone())
    );
    b.rule_2("in <named-month>",
             b.reg(r#"in"#)?,
             time_check!(form!(Form::Month(_))),
             |_, a| Ok(a.value().clone())
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"monday|mon\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"tuesday|tues?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"wed?nesday|wed\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"thursday|thu(?:rs?)?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"friday|fri\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"saturday|sat\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"sunday|sun\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"january|jan\.?"#)?,
                      |_| helpers::month(1)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"february|feb\.?"#)?,
                      |_| helpers::month(2)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"march|mar\.?"#)?,
                      |_| helpers::month(3)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"april|apr\.?"#)?,
                      |_| helpers::month(4)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"may"#)?,
                      |_| helpers::month(5)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"june|jun\.?"#)?,
                      |_| helpers::month(6)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"july|jul\.?"#)?,
                      |_| helpers::month(7)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"august|aug\.?"#)?,
                      |_| helpers::month(8)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"september|sept?\.?"#)?,
                      |_| helpers::month(9)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"october|oct\.?"#)?,
                      |_| helpers::month(10)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"november|nov\.?"#)?,
                      |_| helpers::month(11)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"december|dec\.?"#)?,
                      |_| helpers::month(12)
    );
    b.rule_2("nth sunday of advent",
             ordinal_check!(),
             b.reg(r#"sunday of advent"#)?,
             |ordinal, _| {
                helpers::day_of_week(Weekday::Sun)?.the_nth_after(-(4 - ordinal.value().value) - 1, &helpers::month_day(12, 25)?)
             }
    );
    b.rule_1_terminal("christmas",
                      b.reg(r#"(?:xmas|christmas)(?: day)?"#)?,
                      |_| helpers::month_day(12, 25)
    );
    b.rule_1_terminal("christmas eve",
                      b.reg(r#"(?:xmas|christmas)(?: day)?(?:'s)? eve"#)?,
                      |_| helpers::month_day(12, 24)
    );
    b.rule_1_terminal("new year's eve",
                      b.reg(r#"new year'?s? eve"#)?,
                      |_| helpers::month_day(12, 31)
    );
    b.rule_1_terminal("new year's day",
                      b.reg(r#"new year'?s?(?: day)?"#)?,
                      |_| helpers::month_day(1, 1)
    );
    b.rule_1_terminal("valentine's day",
                      b.reg(r#"valentine'?s?(?: day)?"#)?,
                      |_| helpers::month_day(2, 14)
    );
    b.rule_1_terminal("MLK Day",
                      b.reg(r#"(?:MLK|Martin Luther King,?)(?: Jr.?| Junior)? day"#)?,
                      |_| {
                          let third_week_january =
                              helpers::cycle_nth_after(Grain::Week, 3, &helpers::month_day(1, 1)?)?;
                          let january = helpers::month(1)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?;
                          january.intersect(&third_week_january)?.intersect(&monday)
                      });
    b.rule_1_terminal("Palm sunday",
        b.reg(r#"(?:palm|passion) sunday"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -7, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Thursday",
        b.reg(r#"(?:holy|maundy) thursday"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -3, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Friday",
        b.reg(r#"good friday"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -2, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Saturday",
        b.reg(r#"(?:holy|black) saturday|easter vigil"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, -1, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("Easter",
        b.reg(r#"easter sunday"#)?,
        |_| Ok(helpers::easter()?.form(Form::Celebration))
    );
    b.rule_1_terminal("Easter Monday",
        b.reg(r#"easter monday"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 1, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("Ascension",
        b.reg(r#"(?:(?:the )?feast of (?:the )?)?ascension(?: holiday|thursday|day)?"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 39, &helpers::easter()?)?
                .form(Form::Celebration))

    );
    b.rule_1_terminal("Pentecost",
        b.reg(r#"(?:(?:the )?(?:feast|day) of )?pentecost"#)?,
        |_| Ok(helpers::cycle_nth_after(Grain::Day, 49, &helpers::easter()?)?
                .form(Form::Celebration))
    );
    b.rule_1_terminal("memorial day",
                      b.reg(r#"memorial day"#)?,
                      |_| {
                          let monday = helpers::day_of_week(Weekday::Mon)?;
                          let may = helpers::month(5)?;
                          monday.last_of(&may)
                      });
    b.rule_1_terminal("memorial day weekend",
                      b.reg(r#"memorial day week(?:\s|-)?end"#)?,
                      |_| {
                          let monday = helpers::day_of_week(Weekday::Mon)?;
                          let tuesday = helpers::day_of_week(Weekday::Tue)?;
                          let may = helpers::month(5)?;
                          let start = helpers::cycle_nth_after(Grain::Day, -3, &monday.last_of(&may)?)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let end = tuesday.last_of(&may)?
                              .intersect(&helpers::hour(0, false)?)?;
                          start.span_to(&end, false)
                      });
    b.rule_1_terminal("US independence day",
                      b.reg(r#"(independence|national) day"#)?,
                      |_| helpers::month_day(7, 4)
    );
    b.rule_1_terminal("labor day",
                      b.reg(r#"labor day"#)?,
                      |_| {
                          helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Mon)?)
                      }
    );
    b.rule_1_terminal("flag day",
                      b.reg(r#"flag day"#)?,
                      |_| {
                          helpers::month_day(6, 14)
                      }
    );
    b.rule_1_terminal("patriot day",
        b.reg(r#"patriot day"#)?,
        |_| helpers::month_day(9, 11)
    );
    b.rule_1_terminal("women's equality day",
        b.reg(r#"wom[ea]n'?s equality day"#)?,
        |_| helpers::month_day(8, 26)
    );
    b.rule_1_terminal("labor day weekend",
                      b.reg(r#"labor day week(?:\s|-)?end"#)?,
                      |_| {
                          let start = helpers::cycle_nth_after(Grain::Day, -3, &helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Mon)?)?)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let end = helpers::month(9)?.intersect(&helpers::day_of_week(Weekday::Tue)?)?
                              .intersect(&helpers::hour(0, false)?)?;
                          start.span_to(&end, false)
                      }
    );
    b.rule_1_terminal("Father's Day",
                      b.reg(r#"father'?s?'? day"#)?,
                      |_| {
                          let sundays_of_june = helpers::month(6)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          let second_week_of_june = helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(6, 1)?)?;
                          sundays_of_june.intersect(&second_week_of_june) // third sunday of June
                      }
    );
    b.rule_1_terminal("Mother's Day",
                      b.reg(r#"mother'?s? day"#)?,
                      |_| {
                          let sundays_of_may = helpers::month(5)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          let first_week_of_may = helpers::cycle_nth_after(Grain::Week, 1, &helpers::month_day(5, 1)?)?;
                          sundays_of_may.intersect(&first_week_of_may) // second sunday of May
                      }
    );
    b.rule_1_terminal("halloween day",
                      b.reg(r#"hall?owe?en(?: day)?"#)?,
                      |_| {
                          helpers::month_day(10, 31)
                      }
    );
    b.rule_1_terminal("thanksgiving day",
                      b.reg(r#"thanks?giving(?: day)?"#)?,
                      |_| {
                          let thursday_november = helpers::month(11)?.intersect(&helpers::day_of_week(Weekday::Thu)?)?;
                          let fourth_week_of_november = helpers::cycle_nth_after(Grain::Week, 4, &helpers::month_day(11, 1)?)?;
                          thursday_november.intersect(&fourth_week_of_november) // fourth thursday of november
                      }
    );
    b.rule_1_terminal("black friday",
                      b.reg(r#"black frid?day"#)?,
                      |_| {
                          let thursday_november = helpers::month(11)?.intersect(&helpers::day_of_week(Weekday::Fri)?)?;
                          let fourth_week_of_november = helpers::cycle_nth_after(Grain::Week, 4, &helpers::month_day(11, 1)?)?;
                          thursday_november.intersect(&fourth_week_of_november) // fourth friday of november
                      }
    );
    b.rule_2("absorption of , after named day",
             time_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#","#)?,
             |a, _| Ok(a.value().clone())
    );
    b.rule_1_terminal("now",
                      b.reg(r#"(?:just|right)? ?now|immediately"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );
    b.rule_1_terminal("today",
                      b.reg(r#"todays?|(?:at this time)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"(?:tmrw?|tomm?or?rows?)"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"yesterdays?"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("end of week",
        b.reg(r#"(?:the )?end of (?:the )?week"#)?,
        |_| helpers::day_of_week(Weekday::Thu)
                    ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    b.rule_1_terminal("by the end of week",
        b.reg(r#"by (?:the )?end of (?:the )?week"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)?
                    .span_to(&helpers::day_of_week(Weekday::Sun)?, true)
    );
    b.rule_1_terminal("EOM|End of month",
        b.reg(r#"(?:the )?(?:eom|end of (?:the )?month)"#)?,
        |_| {
            let month = helpers::cycle_nth(Grain::Month, 1)?;
            Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                .span_to(&month, false)?
                .latent()
                .form(Form::PartOfMonth))
        } 
    );
    b.rule_1_terminal("by the end of month",
        b.reg(r#"by (?:the )?(?:eom|end of (?:the )?month)"#)?,
        |_| helpers::cycle_nth(Grain::Second, 0)?
                    .span_to(&helpers::cycle_nth(Grain::Month, 0)?, true)
    );
    b.rule_1_terminal("EOY|End of year",
        b.reg(r#"(?:the )?(?:eoy|end of (?:the )?year)"#)?,
        |_| {
            let current_year = helpers::cycle_nth(Grain::Year, 0)?;
            let start = current_year.intersect(&helpers::month(10)?)?;
            let end = current_year.intersect(&helpers::month(12)?)?;
            start.span_to(&end, true)
        } 
    );
    b.rule_1_terminal("by the end of year",
        b.reg(r#"by (?:the )?(?:eoy|end of (?:the )?year)"#)?,
        |_| {
          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
          let end = current_year.intersect(&helpers::month(12)?)?;
          helpers::cycle_nth(Grain::Second, 0)?
                    .span_to(&end, true)
        }
    );
    b.rule_2("this|next <day-of-week>",
             b.reg(r#"this|next"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             |_, a| {
                 a.value().the_nth_not_immediate(0)
             }
    );
    b.rule_2("this <time>",
             b.reg(r#"the|this|current|coming"#)?,
             time_check!(),
             |_, a| {
                 a.value().the_nth(0)
             }
    );
    b.rule_2("next <time>",
             b.reg(r#"next"#)?,
             time_check!(|time: &TimeValue| !time.latent),
             |_, a| {
                 a.value().the_nth(0)
             }
    );
    b.rule_2("last <time>",
             b.reg(r#"(?:this past|last)"#)?,
             time_check!(),
             |_, a| {
                 a.value().the_nth(-1)
             }
    );
    b.rule_2("<time> after next",
             time_check!(),
             b.reg(r#"after next"#)?,
             |a, _| {
                 a.value().the_nth_not_immediate(1)
             }
    );
    b.rule_2("<time> before last",
             time_check!(),
             b.reg(r#"before last"#)?,
             |a, _| {
                 a.value().the_nth(-2)
             }
    );
    b.rule_4("last <day-of-week> of <time>",
             b.reg(r#"last"#)?,
             time_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#"of"#)?,
             time_check!(),
             |_, a, _, b| {
                 a.value().last_of(b.value())
             }
    );
    b.rule_4("last <cycle> of <time>",
             b.reg(r#"last"#)?,
             cycle_check!(),
             b.reg(r#"of|in"#)?,
             time_check!(),
             |_, cycle, _, time| {
                 cycle.value().last_of(time.value())
             }
    );
    b.rule_4("nth <time> of <time>",
             ordinal_check!(), // the first
             time_check!(), // Thursday
             b.reg(r#"of|in"#)?, // of
             time_check!(), // march
             |ordinal, a, _, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_5("nth <time> of <time>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             time_check!(),
             b.reg(r#"of|in"#)?,
             time_check!(),
             |_, ordinal, a, _, b| {
                 b.value().intersect(a.value())?.the_nth(ordinal.value().value - 1)
             }
    );
    b.rule_4("nth <time> after <time>",
             ordinal_check!(),
             time_check!(),
             b.reg(r#"after"#)?,
             time_check!(),
             |ordinal, a, _, b| {
                 a.value().the_nth_after(ordinal.value().value - 1, b.value())
             }
    );
    b.rule_5("nth <time> after <time>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             time_check!(),
             b.reg(r#"after"#)?,
             time_check!(),
             |_, ordinal, a, _, b| {
                 a.value().the_nth_after(ordinal.value().value - 1, b.value())
             }
    );
    b.rule_1("year",
             integer_check_by_range!(1000, 2100),
             |integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_1("year short",
             integer_check_by_range!(01, 99),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_2("year composed",
             integer_check_by_range!(19, 21),
             integer_check_by_range!(10, 99),
             |a, b| {
                 let y = a.value().value * 100 + b.value().value;
                 Ok(helpers::year(y as i32)?.latent())
             }
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(-1000, 999),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_1("year (latent)",
             integer_check_by_range!(2101, 2200),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_2("the <day-of-month> (ordinal)",
             b.reg(r#"the"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |_, ordinal| {
                 Ok(helpers::day_of_month(ordinal.value().value as u32)?.latent())
             }
    );
    b.rule_1("<day-of-month> (ordinal)",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |ordinal| {
                 Ok(helpers::day_of_month(ordinal.value().value as u32)?.latent())
             }
    );
    b.rule_2("the <day-of-month> (non ordinal)",
             b.reg(r#"the"#)?,
             integer_check_by_range!(1, 31),
             |_, integer| {
                 Ok(helpers::day_of_month(integer.value().value as u32)?.latent())
             }
    );
    b.rule_2("<named-day> <day-of-month> (ordinal)",
             time_check!(form!(Form::DayOfWeek{..})),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |a, ordinal| {
                 a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
             }
    );
    b.rule_2("<named-day> <month-day>",
        time_check!(form!(Form::DayOfWeek{..})),
        time_check!(form!(Form::MonthDay(_))),
        |dow, month_day| {
            month_day.value().intersect(&dow.value())
        }
    );
    b.rule_2("<month-day> <year>",
        time_check!(form!(Form::MonthDay(_))),
        time_check!(form!(Form::Year(_))),
        |month_day, year| {
            year.value().intersect(&month_day.value())
        }
    );
    b.rule_2("<named-month> <day-of-month> (ordinal)", // march 12th
             time_check!(form!(Form::Month{..})),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |month, ordinal| {
                let m = month.value().form_month()?;
                let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: ordinal.value().value as u32}));
                 Ok(month.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?
                    .form(form))

             }
    );
    b.rule_2("<named-month> <day-of-month> (non ordinal)",
             time_check!(form!(Form::Month(_))),
             integer_check_by_range!(1, 31),
             |month, integer| {
                let m = month.value().form_month()?;
                let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
    b.rule_3("<day-of-month> (ordinal) of <named-month>",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             b.reg(r#"of|in"#)?,
             time_check!(form!(Form::Month(_))),
             |ordinal, _, month| {
                let m = month.value().form_month()?;
                let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: ordinal.value().value as u32}));
                Ok(month.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?.form(form))
             }
    );
    b.rule_3("<day-of-month> (non ordinal) of <named-month>",
             integer_check_by_range!(1, 31),
             b.reg(r#"of|in"#)?,
             time_check!(form!(Form::Month(_))),
             |integer, _, month| {
                let m = month.value().form_month()?;
                let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
    b.rule_2("<day-of-month> (non ordinal) <named-month>",
             integer_check_by_range!(1, 31),
             time_check!(form!(Form::Month(_))),
             |integer, month| {
                let m = month.value().form_month()?;
                let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: integer.value().value as u32}));
                Ok(month.value().intersect(&helpers::day_of_month(integer.value().value as u32)?)?.form(form))
             }
    );
    b.rule_2("<day-of-month>(ordinal) <named-month>", //12nd march
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             time_check!(form!(Form::Month(_))),
             |ordinal, month| {
                let m = month.value().form_month()?;
                let form = Form::MonthDay(Some(MonthDayForm { month: m,  day_of_month: ordinal.value().value as u32}));
                Ok(month.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?.form(form))
             }
    );
    // b.rule_3("<day-of-month>(ordinal) <named-month> year",
    //          ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
    //          time_check!(form!(Form::Month(_))),
    //          b.reg(r#"(\d{2,4})"#)?,
    //          |ordinal, a, text_match| {
    //              let year: i32 = text_match.group(1).parse()?;
    //              a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)?.intersect(&helpers::year(year)?)
    //          }
    // );
    b.rule_2("the ides of <named-month>",
             b.reg(r#"the ides? of"#)?,
             time_check!(form!(Form::Month(_))),
             |_, a| {
                 let day = match a.value().form_month()? {
                     3 | 5 | 7 | 10 => 15,
                     _ => 13,
                 };
                 a.value().intersect(&helpers::day_of_month(day)?)
             }
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(1, 23),
             |integer| {
                 Ok(helpers::hour(integer.value().value as u32, integer.value().value <= 12)?.latent())
             }
    );
    b.rule_1("time-of-day (latent)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_1("time-of-day (latent)",
            number_check!(|number: &NumberValue| {
                let hour = (number.value() - 0.5) as u32;
                hour as f32 == (number.value() - 0.5) && hour >= 1 && hour <= 23
            }),
             |number| {
                let hour = number.value().value() as u32;
                Ok(helpers::hour_minute(hour, 30, hour <= 12)?.latent())
             }
    );
    b.rule_2("at <time-of-day>",
             b.reg(r#"at|@"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("<time-of-day> o'clock",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"o.?clock"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_1_terminal("hh:mm",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          true)
    );
    b.rule_1_terminal("hh:mm:ss",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)[:.]([0-5]\d)"#)?,
                      |text_match| helpers::hour_minute_second(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?,
                          true)
    );
    b.rule_2_terminal("hhmm (military) am|pm",
                      b.reg(r#"((?:1[012]|0?\d))([0-5]\d)"#)?,
                      b.reg(r#"([ap])\.?m?\.?"#)?,
                      |a, b| {
                          let day_period = if b.group(1) == "a" {
                              helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                          } else {
                              helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                          };
                          let anchor = helpers::hour_minute(
                              a.group(1).parse()?,
                              a.group(2).parse()?,
                              true)?;
                          let form = anchor.form.clone();
                          Ok(anchor.intersect(&day_period)?.form(form))
                      }
    );
    b.rule_2("<time-of-day> am|pm",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:in the )?([ap])(?:\s|\.)?m?\.?"#)?,
             |a, text_match| {
                 let day_period = if text_match.group(1) == "a" {
                     helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
                 } else {
                     helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
                 };
                 Ok(a.value().intersect(&day_period)?.form(a.value().form.clone()))
             }
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"noon|midday"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight|EOD|end of day",
                      b.reg(r#"midni(?:ght|te)|(?:the )?(?:eod|end of (?:the )?day)"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"(?:a|one)? ?quarter"#)?,
                      |_| Ok(RelativeMinuteValue(15))
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"half"#)?,
                      |_| Ok(RelativeMinuteValue(30))
    );
    b.rule_1("number (as relative minutes)",
             integer_check_by_range!(1, 59),
             |a| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("number <minutes> (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"minutes?"#)?,
             |a, _| Ok(RelativeMinuteValue(a.value().value as i32))
    );
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             relative_minute_check!(),
             |time, relative_minute| Ok(helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 true)?
                .precision(time.value().precision))
    );

    b.rule_3("relative minutes to|till|before <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"to|till|before|of"#)?,
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |relative_minute, _, time| Ok(helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minute.value().0,
                 true)?
                .precision(time.value().precision))
    );

    b.rule_3("relative minutes after|past <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"after|past"#)?,
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |relative_minute, _, time| Ok(helpers::hour_relative_minute(
                 time.value().form_time_of_day()?.full_hour(),
                 relative_minute.value().0,
                 true)?.precision(time.value().precision))
    );

    b.rule_2("half <integer> (UK style hour-of-day)",
             b.reg(r#"half"#)?,
             time_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |_, a| Ok(helpers::hour_relative_minute(
                    a.value().form_time_of_day()?.full_hour(), 
                    30, 
                    true)?.precision(a.value().precision))
    );
    b.rule_1_terminal("mm/.dd/.yyyy",
                      b.reg(r#"(0?[1-9]|1[0-2])[-/.](3[01]|[12]\d|0?[1-9])[-/.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
    b.rule_1_terminal("yyyy-mm-dd",
                      b.reg(r#"(\d{2,4})-(0?[1-9]|1[0-2])-(3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );

    b.rule_1_terminal("mm/dd",
                      b.reg(r#"(0?[1-9]|1[0-2])/(3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
    b.rule_1_terminal("morning",
                      b.reg(r#"morning"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(12, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("breakfast",
        b.reg(r#"breakfast"#)?,
        |_| Ok(helpers::hour(5, false)?
                .span_to(&helpers::hour(10, false)?, false)?
                .latent()
                .form(Form::Meal))
    );
    b.rule_1_terminal("early morning",
                      b.reg(r#"early (?:(?:in|hours of) the )?morning"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(9, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("before work",
        b.reg(r#"before work"#)?,
        |_| {
            let period = helpers::hour(7, false)?
                    .span_to(&helpers::hour(10, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::Morning)))
        }
    );
    b.rule_1_terminal("work",
        b.reg(r#"during work(?: time)?"#)?,
        |_| {
            let period = helpers::hour(9, false)?
                    .span_to(&helpers::hour(19, false)?, false)?;
            Ok(helpers::cycle_nth(Grain::Day, 0)?.intersect(&period)?.form(Form::PartOfDay(PartOfDayForm::None)))
        }
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"after ?noo?n"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("night",
                      b.reg(r#"evening|night"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );

    b.rule_1_terminal("night",
                      b.reg(r#"evening|night"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_1_terminal("brunch",
        b.reg(r#"brunch"#)?,
        |_| Ok(helpers::hour(10, false)?
                .span_to(&helpers::hour(15, false)?, false)?
                .latent()
                .form(Form::Meal))
    );
    b.rule_1_terminal("lunch",
                      b.reg(r#"lunch"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(14, false)?, false)?
                              .latent()
                              .form(Form::Meal))
                      }
    );

    b.rule_1_terminal("dinner",
        b.reg(r#"dinner|supper"#)?,
        |_| Ok(helpers::hour(18, false)?
                .span_to(&helpers::hour(23, false)?, false)?
                .latent()
                .form(Form::Meal))
    );

    b.rule_1_terminal("tea",
        b.reg(r#"(?:at )?tea time"#)?,
        |_| Ok(helpers::hour(15, false)?
                .span_to(&helpers::hour(17, false)?, false)?
                .form(Form::Meal))
    );
    b.rule_2("at <meal>",
        b.reg("at|for|during")?,
        time_check!(form!(Form::Meal)),
        |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("around <meal>",
        b.reg("(?:about|around|approximately)")?,
        time_check!(form!(Form::Meal)),
        |_, a| Ok(a.value().clone().not_latent().precision(Approximate))
    );
    b.rule_2("<meal><time>",
        time_check!(|time: &TimeValue| time.latent && form!(Form::Meal)(time)),
        b.reg("time")?,
        |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_2("in|during the <part-of-day>",
             b.reg(r#"(?:in|during)(?: the)?"#)?,
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, time| Ok(time.value().clone().not_latent())
    );
    b.rule_2("this <part-of-day>",
             b.reg(r#"this"#)?,
             time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
             |_, time| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(time.value())?
                 .form(time.value().form.clone()))
    );
    b.rule_1_terminal("tonight",
                      b.reg(r#"toni(?:ght|gth|te)"#)?,
                      |_| {
                          let period = helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Night)))
                      }
    );
    b.rule_1_terminal("after lunch",
                      b.reg(r#"after(?:-|\s)?lunch"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?.span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("after work",
                      b.reg(r#"after(?:-|\s)?work"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?.span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_2("<time> <part-of-day>",
            time_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
            time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
            |time, part_of_day| time.value().intersect(part_of_day.value())
    );
    b.rule_2("<part-of-day> <time>",
            time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
            time_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
            |part_of_day, time| time.value().intersect(part_of_day.value())
    );
    b.rule_3("<part-of-day> of <time>",
            time_check!(|time: &TimeValue| form!(Form::PartOfDay(_))(time) || form!(Form::Meal)(time)),
            b.reg(r#"of"#)?,
            time_check!(|time: &TimeValue| excluding_form!(Form::Year(_))(time) && excluding_form!(Form::Month(_))(time)),
            |part_of_day, _, time| time.value().intersect(part_of_day.value())
    );
    b.rule_1_terminal("week-end",
                      b.reg(r#"(?:the )?(?:week(?:\s|-)?end|wkend)"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          friday.span_to(&monday, false)
                      }
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?summer"#)?,
                      |_| helpers::month_day(6, 21)?.span_to(&helpers::month_day(9, 23)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?(?:fall|autumn)"#)?,
                      |_| helpers::month_day(9, 23)?.span_to(&helpers::month_day(12, 21)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?winter"#)?,
                      |_| helpers::month_day(12, 21)?.span_to(&helpers::month_day(3, 20)?, false)
    );
    b.rule_1_terminal("season",
                      b.reg(r#"(?:the )?spring"#)?,
                      |_| helpers::month_day(3, 20)?.span_to(&helpers::month_day(6, 21)?, false)
    );
    b.rule_2("<time-of-day> approximately",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:-?ish|approximately)"#)?,
             |time, _| Ok(time.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_1_terminal("<hour>ish",
             b.reg(r#"(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)ish"#)?,
             |text_match| {
                let hour = match text_match.group(1).as_ref() {
                  "one" => 1,
                  "two" => 2,
                  "three" => 3,
                  "four" => 4,
                  "five" => 5,
                  "six" => 6, 
                  "seven" => 7,
                  "eight" => 8,
                  "nine" => 9,
                  "ten" => 10,
                  "eleven" => 11,
                  "twelve" => 12,
                   _ => return Err(RuleError::Invalid.into()),
                };
                Ok(helpers::hour(hour, true)?.precision(Approximate))
    });
    b.rule_2("<time-of-day> sharp",
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:sharp|exactly)"#)?,
             |time, _| Ok(time.value().clone().not_latent().precision(Precision::Exact))
    );
    b.rule_2("about <time-of-day>",
             b.reg(r#"(?:about|around|approximately)"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("exactly <time-of-day>",
             b.reg(r#"exactly"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, time| Ok(time.value().clone().not_latent().precision(Precision::Exact))
    );
    b.rule_4("<month> dd-dd (interval)",
             time_check!(form!(Form::Month(_))),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             |time, a, _, b| {
                 let start = time.value()
                     .intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = time.value()
                     .intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    b.rule_3("<datetime> - <datetime> (interval)",
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"\-|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             |a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_4("from <datetime> - <datetime> (interval)",
             b.reg(r#"from"#)?,
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"\-|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"between"#)?,
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"and"#)?,
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             time_check!(|time: &TimeValue|  !time.latent && form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"\-|:|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("from <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"(?:later than|from)"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"(?:(?:but )?before)|\-|to|th?ru|through|(?:un)?til(?:l)?"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_4("between <time-of-day> and <time-of-day> (interval)",
             b.reg(r#"between"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"and"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_3("<datetime> before <time-of-day> (interval)",
             time_check!(|time: &TimeValue| !time.latent && excluding_form!(Form::TimeOfDay(_))(time)),
             b.reg(r#"until|before|not after"#)?,
             time_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );
    b.rule_2("within <duration>",
             b.reg(r#"within"#)?,
             duration_check!(),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(&a.value().in_present()?, false)
    );
    b.rule_2("by <time>",
             b.reg(r#"by"#)?,
             time_check!(|time: &TimeValue|  !time.latent),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), false)
    );
    b.rule_2("by the end of <time>",
             b.reg(r#"by (?:the )?end of"#)?,
             time_check!(),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), true)
    );
    b.rule_2("until <time>",
             b.reg(r#"(?:anytime |sometimes? )?(?:(?:un)?til(?:l)?|through|up to)"#)?,
             time_check!(),
             |_, a| Ok(a.value().clone().mark_before_end())
    );
    b.rule_2("before <time>",
             b.reg(r#"(?:anytime |sometimes? )?before"#)?,
             time_check!(),
             |_, a| Ok(a.value().clone().mark_before_start())
    );
    b.rule_2("after <time-of-day>",
             b.reg(r#"(?:anytime |sometimes? )?after"#)?,
             time_check!(),
             |_, a| Ok(a.value().clone().mark_after_end())
    );
    b.rule_2("since <time-of-day>",
             b.reg(r#"since"#)?,
             time_check!(),
             |_, a| Ok(a.value().the_nth(-1)?.mark_after_start())
    );
    b.rule_2("about <duration>",
             b.reg(r#"(?:about|around|approximately)"#)?,
             time_check!(|time: &TimeValue|  !time.latent),
             |_, time| Ok(time.value().clone().precision(Precision::Approximate))
    );

    b.rule_2("exactly <duration>",
             b.reg(r#"exactly|precisely"#)?,
             time_check!(|time: &TimeValue|  !time.latent),
             |_, time| Ok(time.value().clone().precision(Precision::Exact))
    );
    Ok(())
}

pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value()));
    b.rule_3("intersect (and X cents)",
             amount_of_money_check!(),
             b.reg(r#"and"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_3("intersect (and number)",
             amount_of_money_check!(),
             b.reg(r#"and"#)?,
             number_check!(),
             |a, _, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1_terminal("$",
                      b.reg(r#"\$|dollars?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("USD",
                      b.reg(r#"us[d\$]|(?:us|american) dollars?|bucks?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"au[d\$]|australian dollars?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("CAD",
                      b.reg(r#"cad|canadian dollars?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_1_terminal("HKD",
                      b.reg(r#"hkd|hk dollars?|hong[- ]?kong dollars?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("HKD") })
    );
    b.rule_1_terminal("EUR",
                      b.reg(r#"€|(?:[e€]uro?s?)"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
                      b.reg(r#"£|pounds?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("GBP",
                      b.reg(r#"gbp|(?:sterling|british) pounds?|sterlings?|quids?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("CHF",
                      b.reg(r#"chf|swiss francs?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
    b.rule_1_terminal("KR",
                      b.reg(r#"kroner?|crowns?|kr"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KR") })
    );
    b.rule_1_terminal("DKK",
                      b.reg(r#"dkk|danish (?:kroner?|crowns?)"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("DKK") })
    );
    b.rule_1_terminal("NOK",
                      b.reg(r#"nok|norwegian (?:kroner?|crowns?)"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("NOK") })
    );
    b.rule_1_terminal("SEK",
                      b.reg(r#"sek|swedish (?:krona|kronor|crowns?)"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("SEK") })
    );
    b.rule_1_terminal("RUB",
                      b.reg(r#"(?:russian )?rubles?|rub"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"inr|rs\.?|(?:indian )?rupees?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("JPY",
                      b.reg(r#"jpy|yens?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("CNY",
                      b.reg(r#"cny|cnh|rmb|yuans?|renmimbis?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CNY") })
    );
    b.rule_1_terminal("¥",
                      b.reg(r#"¥"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
                      b.reg(r#"₩|krw|(?:south[- ]?)?korean wons?|wons?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("฿",
                      b.reg(r#"฿|bitcoins?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"cents?|penn(?:y|ies)|c|¢"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
    b.rule_2("<unit> <amount>",
             money_unit!(),
             number_check!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: b.value().value(),
                     unit: a.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("<amount> <unit>",
             number_check!(),
             money_unit!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: a.value().value(),
                     unit: b.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("about <amount-of-money>",
             b.reg(r#"(?:about|approx(?:\.|imately)?|close to|near(?: to)?|around|almost)"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"exactly|precisely"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    Ok(())
}

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp",
             number_check!(),
             |a| {
                 Ok(TemperatureValue {
                     value: a.value().value(),
                     unit: None,
                     latent: true,
                 })
             });
    b.rule_2("<latent temp> degrees",
             temperature_check!(|temp: &TemperatureValue| temp.latent),
             b.reg(r#"(?:deg(?:ree?)?s?\.?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: true,
                 })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(|temp: &TemperatureValue| temp.latent),
             b.reg(r#"c(?:el[cs]?(?:ius)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Fahrenheit",
             temperature_check!(|temp: &TemperatureValue| temp.latent),
             b.reg(r#"f(?:ah?rh?eh?n(?:h?eit)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Kelvin",
             temperature_check!(),
             b.reg(r#"k(?:elvin)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("kelvin"),
                     latent: false,
                 })
             });
    Ok(())
}

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_3("intersect (with and)",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             b.reg(r#"and"#)?,
             number_check!(),
             |a, _, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_1_terminal("integer (0..19)",
                      b.reg(r#"(none|zilch|naught|nought|nil|zero|one|two|three|fourteen|four|five|sixteen|six|seventeen|seven|eighteen|eight|nineteen|nine|eleven|twelve|thirteen|fifteen|ten)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "none" => 0,
                              "zilch" => 0,
                              "naught" => 0,
                              "nought" => 0,
                              "nil" => 0,
                              "zero" => 0,
                              "one" => 1,
                              "two" => 2,
                              "three" => 3,
                              "four" => 4,
                              "five" => 5,
                              "six" => 6,
                              "seven" => 7,
                              "eight" => 8,
                              "nine" => 9,
                              "ten" => 10,
                              "eleven" => 11,
                              "twelve" => 12,
                              "thirteen" => 13,
                              "fourteen" => 14,
                              "fifteen" => 15,
                              "sixteen" => 16,
                              "seventeen" => 17,
                              "eighteen" => 18,
                              "nineteen" => 19,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, 1)
                      });
    b.rule_1_terminal("single",
                      b.reg(r#"single"#)?,
                      |_| IntegerValue::new_with_grain(1, 1)
    );
    b.rule_1_terminal("a pair",
                      b.reg(r#"a pair(?: of)?"#)?,
                      |_| IntegerValue::new_with_grain(2, 1)
    );
    b.rule_1_terminal("couple",
                      b.reg(r#"(?:a )?couple(?: of)?"#)?,
                      |_| IntegerValue::new_with_grain(2, 1)
    );
    b.rule_1("few", b.reg(r#"(?:a )?few"#)?, |_| {
        Ok(IntegerValue {
            value: 3,
            grain: Some(1),
            precision: Approximate,
            ..IntegerValue::default()
        })
    });
    b.rule_1_terminal("integer (20..90)",
                      b.reg(r#"(twenty|thirty|fou?rty|fifty|sixty|seventy|eighty|ninety)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "twenty" => 20,
                              "thirty" => 30,
                              "fourty" => 40,
                              "forty" => 40,
                              "fifty" => 50,
                              "sixty" => 60,
                              "seventy" => 70,
                              "eighty" => 80,
                              "ninety" => 90,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, 1)
                      });
    b.rule_2("integer 21..99",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check_by_range!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("integer 21..99",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"-"#)?,
             integer_check_by_range!(1, 9),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1_terminal("integer (numeric)",
                      b.reg(r#"(\d{1,18})"#)?,
                      |text_match| IntegerValue::new(text_match.group(0).parse()?));
    b.rule_1_terminal("integer with thousands separator ,",
                      b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: i64 = reformatted_string.parse()?;
                          IntegerValue::new(value)
                      });
    b.rule_2("special composition for missing hundreds like in one twenty two",
             integer_check_by_range!(1, 9),
             integer_check_by_range!(10, 99),
             |a, b| {
                 let value = a.value().value * 100 + b.value().value;
                 IntegerValue::new_with_grain(value, 1)
             });
    
    b.rule_1_terminal("100, 1_000, 1_000_000, 1_000_000_000",
        b.reg(r#"(hundred|thousand|million|billion)s?"#)?,
        |text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "hundred" => (100, 2),
                "thousand" => (1_000, 3),
                "million" => (1_000_000, 6),
                "billion" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            IntegerValue::new_with_grain(value, grain)
        }
    );

    b.rule_2("200..900, 2_000..9_000, 2_000_000..9_000_000_000",
        integer_check_by_range!(1, 999),
        b.reg(r#"(hundred|thousand|million|billion)s?"#)?,
        |integer, text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "hundred" => (100, 2),
                "thousand" => (1_000, 3),
                "million" => (1_000_000, 6),
                "billion" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            IntegerValue::new_with_grain(integer.value().value * value, grain)
        }
    );
    b.rule_1_terminal("dozen",
        b.reg(r#"dozen"#)?,
        |_| Ok(IntegerValue {
                   value: 12,
                   grain: Some(1),
                   group: true,
                   ..IntegerValue::default()
               })
    );
    b.rule_2("number dozen",
            integer_check_by_range!(1, 99),
            integer_check!(|integer: &IntegerValue| integer.group),
            |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     group: true,
                     ..IntegerValue::default()
                 })
    });

    b.rule_1("decimal number", 
        b.reg(r#"(\d*\.\d+)"#)?, 
        |text_match| {
            let value: f32 = text_match.group(0).parse()?;
            Ok(FloatValue {
                value: value,
                ..FloatValue::default()
            })
    });
    b.rule_2("<integer> and a half",
        integer_check!(),
        b.reg(r#"and a half"#)?,
        |integer, _| FloatValue::new(integer.value().value as f32 + 0.5)
    );
    b.rule_3("number dot number",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"dot|point"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| {
                 Ok(FloatValue {
                     value: b.value().value() * 0.1 + a.value().value(),
                     ..FloatValue::default()
                 })
             });
    b.rule_4("number dot number",
         number_check!(|number: &NumberValue| !number.prefixed()),
         b.reg(r#"dot|point"#)?,
         b.reg(r#"(?:(?:oh |zero )*(?:oh|zero))"#)?,
         number_check!(|number: &NumberValue| !number.suffixed()),
         |a, _, zeros, b| {
             let coeff = 10.0_f32.powf(-1.0 * (zeros.group(0).split_whitespace().count() + 1) as f32);
             Ok(FloatValue {
                 value: b.value().value() * coeff + a.value().value(),
                 ..FloatValue::default()
             })
    });
    b.rule_1_terminal("decimal with thousands separator",
                      b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: f32 = reformatted_string.parse()?;
                          Ok(FloatValue {
                              value: value,
                              ..FloatValue::default()
                          })
                      });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|minus\s?|negative\s?"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * -1,
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             value: float.value * -1.0,
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             });
    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[^\W\$€]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
                 let multiplier = match text_match.group(0).as_ref() {
                     "k" => 1000,
                     "m" => 1000000,
                     "g" => 1000000000,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * multiplier,
                             suffixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         let product = float.value * (multiplier as f32);
                         if product.floor() == product {
                             IntegerValue {
                                 value: product as i64,
                                 suffixed: true,
                                 ..IntegerValue::default()
                             }
                                 .into()
                         } else {
                             FloatValue {
                                 value: product,
                                 suffixed: true,
                                 ..float
                             }
                                 .into()
                         }
                     }
                 })
             });
    b.rule_1_terminal("ordinals (first..19th)",
        b.reg(r#"(zeroth|first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|thirteenth|fourteenth|fifteenth|sixteenth|seventeenth|eighteenth|nineteenth)"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "zeroth" => 0,
                "first" => 1,
                "second" => 2,
                "third" => 3,
                "fourth" => 4,
                "fifth" => 5,
                "sixth" => 6,
                "seventh" => 7,
                "eighth" => 8,
                "ninth" => 9,
                "tenth" => 10,
                "eleventh" => 11,
                "twelfth" => 12,
                "thirteenth" => 13,
                "fourteenth" => 14,
                "fifteenth" => 15,
                "sixteenth" => 16,
                "seventeenth" => 17,
                "eighteenth" => 18,
                "nineteenth" => 19,
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new(value))
    });
    b.rule_1_terminal("ordinals (20th...90th)",
        b.reg(r#"(twen|thir|for|fif|six|seven|eigh|nine)tieth"#)?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "twen" => 20,
                "thir" => 30,
                "for" => 40,
                "fif" => 50,
                "six" => 60,
                "seven" => 70,
                "eigh" => 80,
                "nine" => 90,
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new(value))
    });
    b.rule_2("21th..99th",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 9),
             |integer, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             });

    b.rule_3("21th..99th",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"-"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 9),
             |integer, _, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             });

    b.rule_1_terminal("ordinal (100, 1_000, 1_000_000)",
        b.reg(r#"(hundred|thousand|million|billion)th"#)?,
        |text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "hundred" => (100, 2),
                "thousand" => (1_000, 3),
                "million" => (1_000_000, 6),
                "billion" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new_with_grain(value, grain))
        }
    );

    b.rule_2("ordinal (200..900, 2_000..9_000, 2_000_000..9_000_000_000)",
        integer_check_by_range!(1, 999),
        b.reg(r#"(hundred|thousand|million|billion)th"#)?,
        |integer, text_match| {
            let (value, grain) = match text_match.group(1).as_ref() {
                "hundred" => (100, 2),
                "thousand" => (1_000, 3),
                "million" => (1_000_000, 6),
                "billion" => (1_000_000_000, 9),
                _ => return Err(RuleError::Invalid.into()),
            };
            Ok(OrdinalValue::new_with_grain(integer.value().value * value, grain))
        }
    );

    b.rule_2("ordinal (1_1_000..9_999_999_000)",
        integer_check_by_range!(1000, 99_999_999_000),
        ordinal_check!(|ordinal: &OrdinalValue| {
            let grain = ordinal.grain.unwrap_or(0);
            grain == 2 || grain % 3 == 0
        }),
        |integer, ordinal| {
            let grain = ordinal.value().grain.unwrap_or(0);
            let next_grain = (grain / 3) * 3 + 3;
            if integer.value().value % 10i64.pow(next_grain as u32) != 0 { return Err(RuleError::Invalid.into()); }
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );

    b.rule_2("ordinal (101...9_999_999)",
        integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
        ordinal_check_by_range!(1, 99),
        |integer, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_3("ordinal (101...9_999_999)",
        integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
        b.reg(r#"and"#)?,
        ordinal_check_by_range!(1, 99),
        |integer, _, ordinal| {
            Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
        }
    );
    b.rule_1_terminal("ordinal (digits)",
                      b.reg(r#"0*(\d+) ?(st|nd|rd|th)"#)?,
                      |text_match| {
                          let value: i64 = text_match.group(1).parse()?;
                          Ok(OrdinalValue::new(value))
                      });
    b.rule_2("the <ordinal>",
             b.reg(r#"the"#)?,
             ordinal_check!(),
             |_, ordinal| Ok((*ordinal.value()).prefixed()));
    Ok(())
}