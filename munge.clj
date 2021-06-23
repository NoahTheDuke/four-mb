(ns data-munge
  (:require [babashka.fs :as fs]
            [cheshire.core :as json]
            [clojure.string :as str]))

(def files (fs/glob "assets/dungeon-1" "*.json"))

(doseq [filepath (sort-by str files)
        :let [file (json/parse-string (slurp (str filepath)) true)]]
  (let [layout (->> (:layers file)
                    (filter #(= "layout" (:name %)))
                    (first))
        tiles (->> (:data layout)
                   (partition 10)
                   (map-indexed
                     (fn [y line]
                       (->> line
                            (map-indexed vector)
                            (map (fn [[x tile]] [x y (dec tile)])))))
                   (mapcat seq))
        pt->tile (->> tiles
                      (map (fn [[x y tile]] [[x y] tile]))
                      (into {}))
        points (for [y (range 11)
                     x (range 9)]
                 [x y])

        check-fn
        (fn [acc point]
          (if (neg? (get pt->tile point -1))
            acc
            (let [[x y] point
                  base-tile (get pt->tile point)

                  ;; check horizontal
                  horizontal-checked (get-in acc [:horizontal :checked] #{})
                  h-contained? (contains? horizontal-checked point)
                  [h-checked
                   h-result]
                  (loop [cur-x (inc x)
                         checked horizontal-checked
                         result 1]
                    (if (or h-contained?
                            (< 10 cur-x))
                      [checked
                       result]
                      (let [tile (get pt->tile [cur-x y])]
                        (if (= base-tile tile)
                          (recur (inc cur-x)
                                 (conj checked [cur-x y])
                                 (inc result))
                          [checked
                           result]))))

                  ;; check vertical
                  vertical-checked (get-in acc [:vertical :checked] #{})
                  v-contained? (contains? vertical-checked point)
                  [v-checked
                   v-result]
                  (loop [cur-y (inc y)
                         checked vertical-checked
                         result 1]
                    (if (or v-contained?
                            (< 8 cur-y))
                      [checked
                       result]
                      (let [tile (get pt->tile [x cur-y])]
                        (if (= base-tile tile)
                          (recur (inc cur-y)
                                 (conj checked [x cur-y])
                                 (inc result))
                          [checked
                           result]))))

                  acc (cond-> acc
                        (not h-contained?)
                        (update :horizontal assoc point h-result)
                        (not v-contained?)
                        (update :vertical assoc point v-result))

                  ;; comparison
                  bigger (cond
                           ;; Already saw this
                           (and h-contained? v-contained?) :none
                           ;; both are size 1
                           (and (= 1 h-result) (= 1 v-result)) :point
                           ;; vertical is bigger
                           (< h-result v-result) :vertical
                           ;; even or horizontal is bigger
                           :else :horizontal
                           )

                  new-checked
                  (cond
                    (= :vertical bigger)
                    (as-> h-checked it
                      (apply disj it (for [x (range (inc x) 11)]
                                       [x y]))
                      (apply conj it (for [y (range y (+ y v-result))]
                                       [x y]))
                      (apply conj it h-checked)
                      )
                    (= :horizontal bigger)
                    (as-> v-checked it
                      (apply disj it (for [y (range (inc y) 9)]
                                       [x y]))
                      (apply conj it (for [x (range x (+ x h-result))]
                                       [x y]))
                      (apply conj it v-checked))
                    )

                  acc (-> acc
                        (update :horizontal assoc :checked h-checked)
                        (update :vertical assoc :checked v-checked)
                        )
                  ]

              (cond
                (= :vertical bigger)
                (-> acc
                    (update :horizontal dissoc point)
                    (assoc-in [:horizontal :checked] new-checked))
                (= :horizontal bigger)
                (-> acc
                    (update :vertical dissoc point)
                    (assoc-in [:vertical :checked] new-checked))
                (= :point bigger)
                (-> acc
                    (update :horizontal dissoc point)
                    (update :vertical dissoc point)
                    (assoc-in [:points point] 1))
                :else acc))))

        result (reduce
                 check-fn
                 {:horizontal {:checked #{}}
                  :vertical {:checked #{}}
                  :points {}}
                 points)

        {:keys [horizontal vertical points]} result
        horizontal (dissoc horizontal :checked)
        vertical (dissoc vertical :checked)

        template (->> (:layers file)
                      (remove #(= "layout" (:name %)))
                      (first))
        floor-tile (dec (nth (:data template) 11))
        wall-shape (list
                     (not= floor-tile (dec (nth (:data template) 4)))
                     (not= floor-tile (dec (nth (:data template) 39)))
                     (not= floor-tile (dec (nth (:data template) 74)))
                     (not= floor-tile (dec (nth (:data template) 30))))

        pt-str (fn [x y] (format "%2d" (pt->tile [x y])))
        ]
    (when (seq (remove #(neg? (last %)) tiles))
      (println (str
                 "{\n"
                 "\"" (first (fs/split-ext filepath)) "\", (" (str/join ", " wall-shape) "), " floor-tile ";\n"
                 (when-let
                   [rows
                    (some->> points
                             (sort-by first)
                             (map (fn [[[x y]]]
                                    (str "db " (str/join ", " [x y (pt-str x y)]))))
                             (seq)
                             (str/join ";\n"))]
                   (str rows ";\n"))
                 (when-let
                   [rows
                    (some->> horizontal
                             (sort-by first)
                             (map (fn [[[x y] length]]
                                    (str "db " (str/join ", " [x y (pt-str x y) (+ 80 length)]))))
                             (seq)
                             (str/join ";\n"))]
                   (str rows ";\n"))
                 (when-let
                   [rows
                    (some->> vertical
                             (sort-by first)
                             (map (fn [[[x y] length]]
                                    (str "db " (str/join ", " [x y (pt-str x y) (+ 90 length)]))))
                             (seq)
                             (str/join ";\n"))]
                   (str rows ";\n"))
                 "},\n"
                 )))
    ))
