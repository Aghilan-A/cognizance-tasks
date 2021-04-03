<h1>Web Scraper using Rust</h1>
<h3>WEB SCRAPING</h3>

<h3>What?</h3>
<p>Web = website ; scraping = shaving off (in this case data). Getting data from the web page.</p>

<h3>Why?</h3>
<p>Basically an automation to getting piece of information that are large in a website. 

> Example: Getting the first paragraphs of the wikipedia pages of the search like NVIDIA, Football, Summer.
</p>

<h3>How?</h3>
 
* First connect to the website by making a request asking the website to allow you.
* We read the data of the website as an HTML document.
* We scrape the required data by specifying conditions in the code.
* We store the data acquired to a file.

<h3>Rust:</h3>

<p>We use three libraries 'reqwest','scraper','select'. Pretty table for arranging data like a table format. Reqwest makes us connect to the website. Scraper parses the website for the conditions specified. select helps us to select specific CSS classes which are like attached to one another.</p>

<p>In the worldmeters website, the class in which the data of each country is recorded as "even" and "odd" in an alternating pattern starting from the first country 1.
So we must take five even classes and five odd classes( as ten countries are required), and the coressponding class elements like total deaths , total cases , total recovered.
</p>


<h3>Does your program work?</h3>
<p>No. The overall idea has been explained above. The impl future, traits , match, option are some of the terms I couldn't quite understand. I have put a lot of time into this one task, but also wasted lot of the time not knowing what to do. Thanks for such a introduction for such a powerful tool.</p>
