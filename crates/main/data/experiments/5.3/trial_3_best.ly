\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <fis' cis'' a''>2 <fis' cis'' a''>2 <fis' cis'' a''>2 <fis' cis'' a''>2 <a' cis'' e''>2 <bes' d'' f''>2 <b' d'' fis''>2 <g' d'' b''>2
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
