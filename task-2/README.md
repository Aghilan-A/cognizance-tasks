<h1>MathQuiz</h1>

<h4>Approach:</h4>
<p>I know a little bit about C++ but not Java. So the first thing I learned was how to run the Java file in cmd. This gave me around 8 errors which were typos mainly of semicolon. Then I compiled the code and ran it again and got the error of

> Error: class X is public should be declared in a file named X.java

because I copied the file and ran it in a different file named task-2. I learnt that in Java we must declare the same filename for the class we are using.

Then there was this error on fonts, which was because the import line was commented (import is similar to include I guess).
Then label name was not the exact name
>   JLabel questionLabel = new JLabel("Question:");

 the "q" was not the same it was capitalized so had to change it

 Then the program ran successfully then there was this dialogue box named MathQuiz then it had some bugs as mentioned in question.

 >  '+' was '*' and '-' was '/' and vice versa

 This was wrong in both generating formula for internal calculation and calculation done when user entered input.

 Then I noticed that the 'Wrong counter' was supplying count in multiples of 23, so corrected that to +=1 as shown.

 > else {
     wrongCount += 1; // increasing wrong counter
 }

 Then the Start and Stop button names were inverted so made it correct



 >JButton startButton = new JButton("Start");

 >JButton stopButton = new JButton("Stop");

 Before it was Stop for startButton, Start for stopButton.

 The last bit was tough. (At least I hope it was the last). The time was going crazy. For test runs I just gave one input and the time showed like 161 seconds.

 Then corrected it by changing 100 to 1000. (Changing from milliseconds to seconds)

>   long realTime = timeSpent / 1000; // converting to readible time (because it is miliseconds)


 These were the bugs that I could find. And debugged them with best of my knowledge.

 <h6>Note:</h6>
 <p> I used a little help form my father who pointed out some bugs( changing arithmetic in //regenerating formula section, time), but didn't say the solution. 
