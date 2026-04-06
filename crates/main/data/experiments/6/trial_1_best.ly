\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <a' c'' e''>2 <fis' cis'' a''>2 <a' c'' e''>2 <e' g'' bes''>2 <b' ees'' fis''>2 <fis' cis'' a''>2 <e' g'' bes''>2 <e' g'' bes''>2
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
