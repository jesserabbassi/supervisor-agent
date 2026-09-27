use super::*;

pub(super) struct ReportView;
impl ReportView {
    pub(super) fn render(_app: &SupervisorApp) -> Div {
        let mut chart = div().h(px(210.)).flex().items_end().gap_4();
        for (i, h) in [85., 120., 105., 145., 130., 170., 190.].iter().enumerate() {
            chart = chart.child(
                div()
                    .flex_1()
                    .h(px(*h))
                    .bg(rgb(if i % 2 == 0 { BLUE } else { ORANGE }))
                    .rounded_sm(),
            );
        }
        div().flex().flex_col().gap_4().child(div().flex().gap_3().child(stat("Total Revenue","1,245.50 €",BLUE,"↑ 18%")).child(stat("Total Sessions","428",GREEN,"↑ 12%")).child(stat("Average Session Time","2h 18m",YELLOW,"↑ 8%")).child(stat("Active Customers","56",BLUE,"↑ 16%"))).child(panel().p_4().child(section("Revenue Overview · Last 7 Days")).child(div().mt_5().child(chart)).child(label("Apr 12              Apr 13              Apr 14              Apr 15              Apr 16              Apr 17              Apr 18"))).child(div().flex().gap_4().child(panel().p_4().flex_1().child(section("Popular Games")).child(label("1   Counter-Strike 2                        128 sessions")).child(label("2   Valorant                                   96 sessions")).child(label("3   League of Legends                    64 sessions"))).child(panel().p_4().flex_1().child(section("Session Statistics")).child(label("428 total  ·  356 completed  ·  54 paused  ·  18 cancelled"))))
    }
}
