# ec5
ec2 Cost Calculator with Comparison

## What's this?
`ec5` is a local API server to estimate the cost of Amazon EC2 in response to a request of an instance type and its count that you plan to purchase. It reimplements [On-Demand](https://aws.amazon.com/ec2/pricing/on-demand/) and [Reserved](https://aws.amazon.com/ec2/pricing/reserved-instances/pricing/) cost calculations in a more programmable way. 

[![asciicast](/assets/ec5-demo.gif)](https://asciinema.org/a/584587)

## Features
- JSON response
- Comparison with On-Demand/Reserved costs at one time
- Multiple currency(U.S. dollars or Japanese yen)

## Usage

1. Start an ec5 docker container. 
```
$ docker run --rm -p 8000:8000 --name ec5 -it kyagi/ec5:latest
```

2. Send GET request with an instance type and count.
```
➜ xh localhost:8000/t4g.small | jq -r '.on_demand[0]'
{
  "currency": "USD",
  "hourly": "0.0216",
  "per_day": "0.52",
  "per_month": "15.82",
  "per_week": "3.64",
  "per_year": "189.80",
  "quantity": "1"
}

➜ xh localhost:8000/t4g.small?count=3 | jq -r '.on_demand[0]'
{
  "currency": "USD",
  "hourly": "0.0648",
  "per_day": "1.56",
  "per_month": "47.46",
  "per_week": "10.92",
  "per_year": "569.40",
  "quantity": "3"
}
```

## How to expand EC2 pricing data

`400 Bad Request` in response to your request means that the current pricing data does not cover an instance type you request. 

```
➜ xh localhost:8000/m6a.48xlarge
HTTP/1.1 400 Bad Request
Content-Length: 69
Content-Type: application/json
Date: Fri, 12 May 2023 10:43:37 GMT

Something goes wrong... Have you specified unavailable instance type?
```

In the above case, you can expand the pricing data, specifically `ec2pricing.yaml` file by yourself and build your own image to run in the following way.

1. Clone the repository.
```
$ git clone git@github.com:kyagi/ec5.git
$ cd ec5
```

2. Add entries to the pricing data file, then update `InstanceType` enum and `FromStr` trait accordingly.
```
➜ vi ec2pricing.yaml
➜ vi src/ec2pricing.rs 
```

3. Build the customized image and run it.
```
➜ docker buildx build --platform linux/amd64 -t ec5:customized .
➜ # or
➜ docker buildx build --platform linux/arm64 -t ec5:customized .

➜ docker run --rm -p 8000:8000 --name ec5-customized -it ec5:customized
```

## Working examples

### Filter response with jq 
```
➜ xh localhost:8000/c6i.xlarge?count=14 | jq -cr '.on_demand[]|select(.currency == "USD")'
{"currency":"USD","hourly":"2.996","per_day":"71.96","per_month":"2188.76","per_week":"503.72","per_year":"26265.40","quantity":"14"}
```

### Convert response into tables with `ec5` script (embedded in the repository)
```
➜ ./ec5 t4g.large
# OnDemand cost
  daily    monthly   annually    upfront       term    initial quantity
   2.07         62        755        n/a        n/a        n/a        1

# Reserved costs
  daily    monthly   annually    upfront       term    initial quantity
   1.31         39        478         No    OneYear          0        1
   1.24         37        452    Partial    OneYear        227        1
   1.22         37        445        All    OneYear        445        1
   0.90         27        328         No ThreeYears          0        1
   0.83         25        302    Partial ThreeYears        455        1
   0.78         23        284        All ThreeYears        855        1
```
```
➜ ./ec5 t4g.large 3
# OnDemand cost
  daily    monthly   annually    upfront       term    initial quantity
   6.21        188      2,266        n/a        n/a        n/a        3

# Reserved costs
  daily    monthly   annually    upfront       term    initial quantity
   3.93        119      1,434         No    OneYear          0        3
   3.72        113      1,357    Partial    OneYear        681        3
   3.66        111      1,335        All    OneYear      1,335        3
   2.70         82        985         No ThreeYears          0        3
   2.49         75        908    Partial ThreeYears      1,365        3
   2.34         71        854        All ThreeYears      2,565        3
```
```
➜ ./ec5 t4g.large 3 JPY
# OnDemand cost
  daily    monthly   annually    upfront       term    initial quantity
844.560     25,687    308,264        n/a        n/a        n/a        3

# Reserved costs
  daily    monthly   annually    upfront       term    initial quantity
534.480     16,258    195,085         No    OneYear          0        3
505.920     15,389    184,660    Partial    OneYear     92,616        3
497.760     15,140    181,682        All    OneYear    181,560        3
367.200     11,171    134,028         No ThreeYears          0        3
338.640     10,302    123,603    Partial ThreeYears    185,640        3
318.240      9,677    116,157        All ThreeYears    348,840        3
```