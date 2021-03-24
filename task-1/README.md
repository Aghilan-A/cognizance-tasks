<h2>Elephant problem</h2>

<h6>Understanding the problem:</h6>
<p>The elephant can go maximum of 5 steps per move. To find the minimum number of moves needed for the elephant to reach x.

<h6>Devising logic:</h6>
<p>As the maximum number of steps per move is 5 we can divide x by 5 to find the minimum steps to get closer. We will look with an example.

> Example 1 : If x = 20, the minimum number of moves is 20/5=4
<p>But it must satisfy all values of x.

> Example 2 : If x = 1199 , 1199/5=239 with a remainder of 4

<p> From that we could see that the remainder is covered by the steps of elephant (1,2,3,4,5 steps per move). Therefore, if the remainder is not zero, then the minimum number of moves is (quotient when divided by 5)+1. Else it is simply the quotient.
