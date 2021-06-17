(ns convert
  (:require [cheshire.core :as json]
            [babashka.fs :as fs]))

(def old->new
  {
    8  1 ;; Black tile
   54  2 ;; Dark gray tile
   22  3 ;; Light gray tile
   ; -1  4 ;; White
   12  5 ;; Dirt floor
   11  6 ;; Mosaic floor
   10  7 ;; Cracked floor
   51  8 ;; Top wall
   43  9 ;; Bottom wall
   34 10 ;; Left wall
   33 11 ;; Right wall
   37 12 ;; Top cracked wall
   36 13 ;; Bottom cracked wall
   38 14 ;; Left cracked wall
   35 15 ;; Right cracked wall
   50 16 ;; Top left inside corner
   49 17 ;; Top right inside corner
   42 18 ;; Bottom left inside corner
   41 19 ;; Bottom right inside corner
   44 20 ;; Top left outside corner
   45 21 ;; Top right outside corner
   52 22 ;; Bottom left outside corner
   53 23 ;; Bottom right outside corner
   17 24 ;; Top door
   18 25 ;; Right door
   19 26 ;; Bottom door
   20 27 ;; Left door
   25 28 ;; Top locked door
   26 29 ;; Right locked door
   27 30 ;; Bottom locked door
   28 31 ;; Left locked door
   21 32 ;; Boss door
    1 33 ;; Block
    2 34 ;; Locked block
    3 35 ;; Fire pit
   29 36 ;; Locked chest
   30 37 ;; Opened chest
    4 38 ;; Gargoyle
   46 39 ;; Top stairs
   ; -1 40 ;; Right stairs
   ; -1 41 ;; Bottom stairs
   ; -1 42 ;; Left stairs
    5 43 ;; Top pit
    7 44 ;; Bottom pit
    6 45 ;; Both pit
   13 46 ;; Top/bottom broken door
   14 47 ;; Left/right broken door
   24 48 ;; Top left railing
   23 49 ;; Top right railing
   16 50 ;; Bottom left railing
   15 51 ;; Bottom right railing
   31 52 ;; Top left/right railing
   32 53 ;; Top/bottom right railing
   40 54 ;; Bottom left/right railing
   39 55 ;; Top/bottom left railing
   47 56 ;; Top left/right, bottom left railing
   48 57 ;; Top left/right, bottom right railing
   56 58 ;; Top right, bottom left/right railing
   55 59 ;; Top left, bottom left/right railing
    9 60 ;; Button
   })

(def files (map str (fs/glob "assets/dungeon-1" "*.json")))

(doseq [filepath files
        :let [file (json/parse-string (slurp filepath) true)]]
  (let [new-file
        (->> (:layers file)
             (map (fn [layer]
                    (assoc layer
                           :data (mapv
                                   #(get old->new % 0)
                                   (:data layer)))))
             (assoc file :layers))]
    (spit filepath
          (json/generate-string new-file))))
