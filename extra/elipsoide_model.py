from sklearn.cluster import KMeans

import pandas as pd

class ElipsoideModel():
    def __init__(self, n_init, tol):
        self.elipsoide_params = {}
        self.cluster_centers = []
        self.number_init = n_init
        self.elipsoide_tol = tol
    
    def fit(self, X, y):
        classes = pd.unique(y["label"])
        classes_order = []
        for c in classes:
            kmeanModel = KMeans(n_clusters=1, n_init=self.number_init).fit(X.loc[y['label'] == c])
            centroids = kmeanModel.cluster_centers_
            self.cluster_centers.append(centroids)
            classes_order.append(c)
        
        i = 0
        for c in classes_order: 
            centroid = self.cluster_centers[i][0]
            class_cluster = X.loc[y['label'] == c]
            features = class_cluster.columns.values
            semiaxis = []
            j = 0
            for feature in features:
                semiaxis.append(class_cluster[feature].max() - centroid[j])
                j += 1
            
            i += 1
             
            self.elipsoide_params[str(c)] = {"center": centroid, "semiaxis": semiaxis}

    def get_cluster_centers(self):
        return self.cluster_centers

    def get_degree(self, point):
        alligiance = {}
        for c in self.elipsoide_params:
            sum = 0
            for i in range(len(point)):
                num = (point[i] - self.elipsoide_params[c]["center"][i])**2
                den = (self.elipsoide_params[c]["semiaxis"][i])**2
                sum += num/den
            
            alligiance[c] = (self.elipsoide_tol - sum)/self.elipsoide_tol
        
        return alligiance