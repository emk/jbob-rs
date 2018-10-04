'((dethm atom/cons (x y) (equal (atom (cons x y)) 'nil))
 (dethm car/cons (x y) (equal (car (cons x y)) x))
 (dethm cdr/cons (x y) (equal (cdr (cons x y)) y))
 (dethm equal-same (x) (equal (equal x x) 't))
 (dethm equal-swap (x y) (equal (equal x y) (equal y x)))
 (dethm if-same (x y) (equal (if x y y) y))
 (dethm if-true (x y) (equal (if 't x y) x))
 (dethm if-false (x y) (equal (if 'nil x y) y))
 (dethm if-nest-E (x y z) (if x 't (equal (if x y z) z)))
 (dethm if-nest-A (x y z) (if x (equal (if x y z) y) 't))
 (dethm cons/car+cdr (x) (if (atom x) 't (equal (cons (car x) (cdr x)) x)))
 (dethm equal-if (x y) (if (equal x y) (equal x y) 't))
 (dethm natp/size (x) (equal (natp (size x)) 't))
 (dethm size/car (x) (if (atom x) 't (equal (< (size (car x)) (size x)) 't)))
 (dethm size/cdr (x) (if (atom x) 't (equal (< (size (cdr x)) (size x)) 't)))
 (dethm associate-+ (a b c) (equal (+ (+ a b) c) (+ a (+ b c))))
 (dethm commute-+ (x y) (equal (+ x y) (+ y x)))
 (dethm natp/+ (x y) (if (natp x) (if (natp y) (equal (natp (+ x y)) 't) 't) 't))
 (dethm positives-+ (x y) (if (< '0 x) (if (< '0 y) (equal (< '0 (+ x y)) 't) 't) 't))
 (dethm common-addends-< (x y z) (equal (< (+ x z) (+ y z)) (< x y)))
 (dethm identity-+ (x) (if (natp x) (equal (+ '0 x) x) 't))
 (defun list-induction (x) (if (atom x) '() (cons (car x) (list-induction (cdr x)))))
 (defun
  star-induction
  (x)
  (if (atom x) x (cons (star-induction (car x)) (star-induction (cdr x)))))
 (defun pair (x y) (cons x (cons y '())))
 (defun first-of (x) (car x))
 (defun second-of (x) (car (cdr x)))
 (dethm first-of-pair (a b) (equal (first-of (pair a b)) a))
 (dethm second-of-pair (a b) (equal (second-of (pair a b)) b))
 (defun in-pair? (xs) (if (equal (first-of xs) '?) 't (equal (second-of xs) '?)))
 (dethm in-first-of-pair (b) (equal (in-pair? (pair '? b)) 't))
 (dethm in-second-of-pair (a) (equal (in-pair? (pair a '?)) 't))
 (defun list0? (x) (equal x '()))
 (defun list1? (x) (if (atom x) 'nil (list0? (cdr x))))
 (defun list2? (x) (if (atom x) 'nil (list1? (cdr x))))
 (defun list? (x) (if (atom x) (equal x '()) (list? (cdr x))))
 (defun sub (x y) (if (atom y) (if (equal y '?) x y) (cons (sub x (car y)) (sub x (cdr y)))))
 (defun memb? (xs) (if (atom xs) 'nil (if (equal (car xs) '?) 't (memb? (cdr xs)))))
 (defun
  remb
  (xs)
  (if (atom xs) '() (if (equal (car xs) '?) (remb (cdr xs)) (cons (car xs) (remb (cdr xs))))))
 (dethm memb?/remb0 () (equal (memb? (remb '())) 'nil))
 (dethm memb?/remb1 (x1) (equal (memb? (remb (cons x1 '()))) 'nil))
 (dethm memb?/remb2 (x1 x2) (equal (memb? (remb (cons x2 (cons x1 '())))) 'nil))
 (dethm memb?/remb (xs) (equal (memb? (remb xs)) 'nil))
 (defun ctx? (x) (if (atom x) (equal x '?) (if (ctx? (car x)) 't (ctx? (cdr x)))))
 (dethm ctx?/t (x) (if (ctx? x) (equal (ctx? x) 't) 't))
 (dethm ctx?/sub (x y) (if (ctx? x) (if (ctx? y) (equal (ctx? (sub x y)) 't) 't) 't))
 (defun member? (x ys) (if (atom ys) 'nil (if (equal x (car ys)) 't (member? x (cdr ys)))))
 (defun set? (xs) (if (atom xs) 't (if (member? (car xs) (cdr xs)) 'nil (set? (cdr xs)))))
 (defun
  add-atoms
  (x ys)
  (if (atom x) (if (member? x ys) ys (cons x ys)) (add-atoms (car x) (add-atoms (cdr x) ys))))
 (defun atoms (x) (add-atoms x '()))
 (dethm set?/t (xs) (if (set? xs) (equal (set? xs) 't) 't))
 (dethm set?/nil (xs) (if (set? xs) 't (equal (set? xs) 'nil)))
 (dethm set?/add-atoms (a bs) (if (set? bs) (equal (set? (add-atoms a bs)) 't) 't))
 (dethm set?/atoms (a) (equal (set? (atoms a)) 't))
 (defun rotate (x) (cons (car (car x)) (cons (cdr (car x)) (cdr x))))
 (dethm rotate/cons (x y z) (equal (rotate (cons (cons x y) z)) (cons x (cons y z))))
 (defun wt (x) (if (atom x) '1 (+ (+ (wt (car x)) (wt (car x))) (wt (cdr x)))))
 (dethm natp/wt (x) (equal (natp (wt x)) 't))
 (dethm positive/wt (x) (equal (< '0 (wt x)) 't))
 (defun
  align
  (x)
  (if (atom x) x (if (atom (car x)) (cons (car x) (align (cdr x))) (align (rotate x)))))
 (dethm align/align (x) (equal (align (align x)) (align x))))
