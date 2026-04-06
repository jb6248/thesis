\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <g' d'' bes''>2 <a' c'' e''>2 <fis' cis'' a''>2 <d' fis'' a''>2 <fis' cis'' bes''>2 <fis' cis'' a''>2 <c' e'' g''>2 <c' e'' g''>2
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
