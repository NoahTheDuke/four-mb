(ns gen-maps
  (:require [babashka.fs :as fs]))

(def template (fs/file "template.json"))

(defn gen-filename [x y]
  (str x "-" y ".json"))

(doseq [x (range 8)
        y (range 8)
        :when (not (fs/exists? (gen-filename x y)))]
  (fs/copy template (gen-filename x y)))
