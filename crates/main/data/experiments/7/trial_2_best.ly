\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <a' cis'' e''>2 <ees' fis'' bes''>2 <a' c'' e''>2 <d' f'' a''>2 <e' g'' b''>2 <ees' fis'' bes''>2 <ees' fis'' bes''>2 <c' e'' g''>2
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
