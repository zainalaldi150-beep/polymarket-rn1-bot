import json
import pandas as pd
import numpy as np
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import accuracy_score
import joblib

def generate_synthetic_data(n_samples=50000):
    np.random.seed(42)
    data = {
        'home_elo': np.random.normal(1500, 100, n_samples),
        'away_elo': np.random.normal(1500, 100, n_samples),
        'home_goals_avg': np.random.exponential(1.5, n_samples),
        'away_goals_avg': np.random.exponential(1.2, n_samples),
        'home_possession': np.random.normal(50, 10, n_samples),
        'away_possession': np.random.normal(50, 10, n_samples),
        'home_shots': np.random.poisson(10, n_samples),
        'away_shots': np.random.poisson(8, n_samples),
        'minute': np.random.randint(0, 95, n_samples),
        'score_diff': np.random.normal(0, 1.5, n_samples),
        'home_win': np.random.binomial(1, 0.4, n_samples)
    }
    return pd.DataFrame(data)

def train_model():
    print("🧠 Training AI Model...")
    df = generate_synthetic_data(50000)
    features = ['home_elo', 'away_elo', 'home_goals_avg', 'away_goals_avg',
                'home_possession', 'away_possession', 'home_shots', 'away_shots',
                'minute', 'score_diff']
    X = df[features]
    y = df['home_win']
    X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)
    scaler = StandardScaler()
    X_train_scaled = scaler.fit_transform(X_train)
    X_test_scaled = scaler.transform(X_test)
    model = RandomForestClassifier(n_estimators=100, max_depth=10, random_state=42, n_jobs=-1)
    model.fit(X_train_scaled, y_train)
    y_pred = model.predict(X_test_scaled)
    acc = accuracy_score(y_test, y_pred)
    print(f"✅ Model accuracy: {acc:.4f}")
    model_data = {
        'feature_names': features,
        'scaler_mean': scaler.mean_.tolist(),
        'scaler_scale': scaler.scale_.tolist(),
        'feature_importances': model.feature_importances_.tolist(),
        'n_estimators': 100,
        'max_depth': 10,
        'accuracy': float(acc)
    }
    with open('../model_weights.json', 'w') as f:
        json.dump(model_data, f, indent=2)
    joblib.dump(model, '../model.pkl')
    joblib.dump(scaler, '../scaler.pkl')
    print("📁 Model saved to model_weights.json, model.pkl, scaler.pkl")

if __name__ == "__main__":
    train_model()
