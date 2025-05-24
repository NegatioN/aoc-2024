(ns day1
  (:require [clojure.string :as str]))

;;(def data "3   4\n4   3\n2   5\n1   3\n3   9\n3   3")
(def data (slurp "../data/day1.txt"))

(defn parse-data [data]
  (->> (str/split data #"\n")
       (map #(str/split % #"\s+"))
       (map #(map read-string %))
       (map vec)
       ))

(defn transpose [vecs]
  (apply map vector vecs) )

(def vecs (transpose (parse-data data)))

(def out (let [[left right] (vec vecs)]
   (vec (map vector (sort left)  (sort right)))))

(defn absolute-difference [pair]
  (Math/abs (- (first pair) (second pair))))

(reduce + ( map absolute-difference out))
