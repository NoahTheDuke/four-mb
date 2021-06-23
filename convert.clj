(ns convert
  (:require [babashka.fs :as fs]
            [clojure.java.shell :refer [sh]]))

(def files (map str (fs/glob "assets/dungeon-1" "*.json")))

(doseq [filepath files]
  (sh "jj" "-u" "-i" filepath "-o" filepath))
