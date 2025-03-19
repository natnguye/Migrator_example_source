def power_function(a, b):
    result=1
    for i in range(b):
        result=result*a
        print "le resultat de %d ^ %d = %d" % (a, b, result)
    return result

a=2
b=8
res=power_function(a, b)

print "le resultat final est %d" % (res)
