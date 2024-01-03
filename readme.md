# TF-IDF

Karen Spärck Jones (1972) conceived a statistical interpretation of term-specificity called Inverse Document Frequency (idf), which became a cornerstone of term weighting:[3]

The specificity of a term can be quantified as an inverse function of the number of documents in which it occurs.
For example, the tf and idf for some words in Shakespeare's 37 plays are as follows

- | Word     | tf  | idf   |
  | -------- | --- | ----- |
  | Romeo    | 1   | 1.57  |
  | salad    | 2   | 1.27  |
  | Falstaff | 4   | 0.967 |
  | forest   | 12  | 0.489 |
  | battle   | 21  | 0.246 |
  | wit      | 34  | 0.037 |
  | fool     | 36  | 0.012 |
  | good     | 37  | 0     |
  | sweet    | 37  | 0     |

#### Statisitcs

- The tf–idf is the product of two statistics, term frequency and inverse document frequency.

- There are various ways for determining the exact values of both statistics.

- A formula that aims to define the importance of a keyword or phrase within a document or a web page.

**Term frequency**
Term frequency, $\mathrm{tf}(t, d)$, is the relative frequency of term $t$ within document $d$,

$$
\operatorname{tf}(t, d)=\frac{f_{t, d}}{\sum_{t^{\prime} \in d} f_{t^{\prime}, d}},
$$

where $f_{t, d}$ is the raw count of a term in a document, i.e., the number of times that term $t$ occurs in document $d$. Note the denominator is simply the total number of terms in document $d$ (counting each occurrence of the same term separately).

**Inverse Document Frequency**

The inverse document frequency is a measure of how much information the word provides, i.e., if it is common or rare across all documents. It is the logarithmically scaled inverse fraction of the documents that contain the word (obtained by dividing the total number of documents by the number of documents containing the term, and then taking the logarithm of that quotient):

$$
\operatorname{idf}(t, D)= 1 + \log \frac{N}{1 + |\{d \in D: t \in d\}|}
$$
