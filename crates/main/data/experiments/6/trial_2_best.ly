\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <g' d'' bes''>2 <c' e'' g''>2 <aes' ees'' b''>2 <g' d'' bes''>2 <g' d'' bes''>2 <a' cis'' e''>2 <a' cis'' e''>2 <a' c'' e''>2
        }
        \bar "|."
      }
    >>
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
