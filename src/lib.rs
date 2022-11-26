pub fn create_tournement<'a, T>(
    list_ref: &'a Vec<T>,
    nb_rounds: usize,
) -> Vec<Vec<(&'a T, &'a T)>> {
    let nb_teams = list_ref.len();
    if nb_teams == 0 {
        return vec![];
    }
    let is_even = nb_teams % 2 == 0;
    let stop = nb_teams - if is_even { 2 } else { 1 };
    let middle = stop / 2;
    let limit = nb_teams - 1;
    let threshold = if is_even { limit } else { nb_teams };
    let nb_days = threshold * nb_rounds;
    let nb_fixtures = middle + if is_even { 1 } else { 0 };

    let mut matches = vec![vec![(&list_ref[0], &list_ref[0]); nb_fixtures]; nb_days];

    for d in 0..nb_days {
        let home_target = (d / threshold) % 2;

        for m in 0..middle {
            let a = (m + d) % threshold;
            let b = (stop - 1 - m + d) % threshold;
            matches[d][m] = if m % 2 == home_target {
                (&list_ref[a], &list_ref[b])
            } else {
                (&list_ref[b], &list_ref[a])
            };
        }

        if is_even {
            let a = (limit - 1 + d) % threshold;
            matches[d][middle] = if d % 2 == 0 {
                (&list_ref[a], &list_ref[limit])
            } else {
                (&list_ref[limit], &list_ref[a])
            }
        }
    }
    matches
}
