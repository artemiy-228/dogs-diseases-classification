# Choosing model

Before creating the application, I had decided how my program will work.
I had to create a program to classify a disease by input options — in the current case, symptoms.

I had several options: expert system and ML/AI models.

In the beginning, I had only a set of rules that looked like this:
```
symptom1, symptom2, symptom3 -> disease1
```



At first glance, this task is obvious. We have to ask all symptoms from the user, but if we dig deeper, we can find several difficult obstacles that are almost unsolvable in this implementation.

### Problem 1
```
If we look at a disease, we will clarify that often not all symptoms appear.
Furthermore, for every disease we have several crucial symptoms and others are additional. 
It means that if a human didn't notice 1 or 2 symptoms, our program won't set their disease correctly.
```


### Problem 2
```
On the other hand, what if a human has 1 or 2 symptoms which are not associated with their disease? 
For example, blood pressure or temperature. Our program cannot solve this either.
```


These 2 problems are almost unsolvable in a classic expert system, so I've decided not to use it and to use ML/AI models instead.

## ML/AI Approach

With ML/AI models, I've seen new obstacles — all of those obstacles are associated with the fact that I had no training dataset.
I only had a set of rules.

Because of that, I decided to create a synthetic dataset, and for its creation, I had to weight all symptoms.

There are several crucial symptoms and others are additional, so I opened the internet and started analyzing information about all of these diseases.

After analyzing, I created a dictionary with all diseases and their weighted symptoms:

```
diseases = {
    "Canine Infectious Hepatitis": {
        "muscle tremors": 1.0,
        "conjunctivitis": 1.0,
        "respiratory system involvement": 1.0,
        "high fever": 0.6,
        "gastrointestinal problems": 0.7,
    },
    "Canine Distemper": {
        "epileptic seizures": 1.0,
        "coordination disorders": 1.0,
        "high fever": 0.6,
        "severe depression": 0.8,
        "lethargy": 0.8,
        "muscle tremors": 0.7,
        "fearfulness": 0.8,
        "serous then purulent nasal discharge": 0.9,
        "serous then purulent eye discharge": 0.9,
        "pustular rash": 0.85,
        "vomiting attacks": 0.8,
        "gastrointestinal problems": 0.7,
    },
    "Parvovirus Family": {
        "nausea and vomiting with blood": 1.0,
        "dehydration": 1.0,
        "high fever": 0.6,
        "lethargy": 0.8,
        "gastrointestinal problems": 0.7,
        "respiratory system involvement": 0.8,
    },
    "Rabies": {
        "hallucinations": 1.0,
        "aggressiveness": 1.0,
        "desire to run": 1.0,
        "fearfulness": 0.8,
        "muscle paralysis": 0.9,
        "difficulty swallowing": 0.85,
        "abnormal appetite": 0.85,
        "excessive salivation": 0.95,
    },
    "Aujeszky's Disease": {
        "convulsions": 1.0,
        "muscle paralysis": 1.0,
        "severe itching and scratching": 1.0,
        "high fever": 0.6,
    },
    "Herpesvirus Family": {
        "conjunctivitis": 1.0,
        "blurred vision": 1.0,
        "severe depression": 0.8,
        "gastrointestinal problems": 0.7,
        "respiratory system involvement": 0.8,
        "anorexia": 0.85,
        "constant crying": 0.9,
    },
    "Influenza Virus": {
        "conjunctivitis": 1.0,
        "high fever": 0.8,
        "lethargy": 0.8,
        "serous then purulent nasal discharge": 0.9,
        "refusal to eat": 0.85,
        "respiratory system involvement": 0.8,
    },
    "Coronavirus": {
        "gastrointestinal problems": 1.0,
    },
}
```


## Dataset Generation

Now I need to create a generator with possibilities for every disease, and for every disease I have to generate at least 200 instances.

So, **Problem 1** has been successfully solved.

It's time to solve **Problem 2** — and furthermore, this problem has been solved too.

To solve it, I collected all symptoms in one array:
```
all_symptoms_en = [
    "high fever",
    "severe depression",
    "lethargy",
    "muscle tremors",
    "hallucinations",
    "fearfulness",
    "aggressiveness",
    "desire to run",
    "CNS involvement",
    "convulsions",
    "epileptic seizures",
    "muscle paralysis",
    "coordination disorders",
    "serous then purulent nasal discharge",
    "serous then purulent eye discharge",
    "conjunctivitis",
    "blurred vision",
    "pustular rash",
    "vomiting attacks",
    "difficulty swallowing",
    "abnormal appetite",
    "nausea and vomiting with blood",
    "dehydration",
    "gastrointestinal problems",
    "refusal to eat",
    "excessive salivation",
    "respiratory system involvement",
    "anorexia",
    "constant crying",
    "severe itching and scratching",
]
```

And now all I need to do is just add 2–5 additional symptoms as noise. That's all!

From now I have a dataset, and it's time to choose a model.

## Model Selection

I have decided to use Random Forest Classifier, because in my case of synthetic dataset, even a fully connected neural network is excessive.

## Dataset Creation Code
```
with open("diseases.csv", "w", newline="", encoding="utf-8") as file:
    writer = csv.writer(file)
    writer.writerow(all_symptoms + ["disease"])

    for i in range(5000):
        disease, symptoms = random.choice(list(diseases.items()))
        row = [0] * len(all_symptoms)
        
        for symptom, weight in symptoms.items():
            row[all_symptoms.index(symptom)] = 1 if random.random() < weight else 0

        noise_symptoms = random.sample(
            [s for s in all_symptoms if s not in symptoms], 
            k=random.randint(2, 5)
        )
        for ns in noise_symptoms:
            if random.random() < 0.2:
                row[all_symptoms.index(ns)] = 1

        row.append(disease)
        writer.writerow(row)
```


## Summary

I have chosen a machine learning model for classification. After thinking, I decided to use Random Forest Classifier.

## Model Training

I won't describe you way of training, i just recommend you to read about sklearn and training classificators

model training is on: Model_training