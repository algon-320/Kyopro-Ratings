# Kyopro-Ratings

以下の競プロサイトのレーティングをjsonで返す

- AtCoder
- Codeforces
- TopCoder

## リクエスト

以下の形式でGETリクエストを送ると結果のjsonを返します。
```
https://kyopro-ratings.herokuapp.com/json?atcoder=AtCoderのユーザ名&codeforces=Codeforcesのユーザ名&topcoder_algorithm=TopCoderのユーザ名&topcoder_marathon=TopCoderのユーザ名
```

### 例

```
$ curl "https://kyopro-ratings.herokuapp.com/json?atcoder=algon&codeforces=algon_320&topcoder_algorithm=algon_320&topcoder_marathon=algon_320"
```

実行結果

```
{
  "atcoder": {
    "color": "#0000FF",
    "rating": 1872,
    "status": "success"
  },
  "codeforces": {
    "color": "#AA00AA",
    "rating": 1937,
    "status": "success"
  },
  "topcoder_algorithm": {
    "color": "#6666FF",
    "rating": 1285,
    "status": "success"
  },
  "topcoder_marathon": {
    "color": "#00A900",
    "rating": 1003,
    "status": "success"
  }
}
```