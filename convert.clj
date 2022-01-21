(ns convert
  (:require [net.cgrand.xforms :as x])
  ; (:require [babashka.fs :as fs]
  ;           [clojure.java.shell :refer [sh]])
  )

; (def files (map str (fs/glob "assets/dungeon-1" "*.json")))

; (doseq [filepath files]
;   (sh "jj" "-u" "-i" filepath "-o" filepath))


(transduce
  (comp
    (x/for [i %
            j (range i)]
      (* i j))
    (filter #(< % 129))
    )
  + 0
  (range 256))

(reduce
  + 0
  (for [
        ; i (range 256)
        i (filter #(< % 129) (range 256))
        j (range i)]
    (* i j)))
