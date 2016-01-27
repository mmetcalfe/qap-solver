results_bma <- read.table("results_bma.tsv",header=T)
results_ea <- read.table("results_ea.tsv",header=T)
results_its <- read.table("results_its.tsv",header=T)
results_sa <- read.table("results_sa.tsv",header=T)

# results_bma$problem_size <- as.numeric(gsub("\\D", "", results_bma$instance_name))
# results_ea $problem_size <- as.numeric(gsub("\\D", "", results_ea $instance_name))
# results_its$problem_size <- as.numeric(gsub("\\D", "", results_its$instance_name))
# results_sa $problem_size <- as.numeric(gsub("\\D", "", results_sa $instance_name))

results_bma <- results_bma[results_bma$instance_name!='esc16f', ]
results_ea  <- results_ea [results_ea $instance_name!='esc16f', ]
results_its <- results_its[results_its$instance_name!='esc16f', ]
results_sa  <- results_sa [results_sa $instance_name!='esc16f', ]

# results_bma[results_bma$instance_name=='esc32a',]$best_known_solution_value = 130
# results_ea [results_ea $instance_name=='esc32a',]$best_known_solution_value = 130
# results_its[results_its$instance_name=='esc32a',]$best_known_solution_value = 130
# results_sa [results_sa $instance_name=='esc32a',]$best_known_solution_value = 130
#
# results_bma[results_bma$instance_name=='esc32b',]$best_known_solution_value = 168
# results_ea [results_ea $instance_name=='esc32b',]$best_known_solution_value = 168
# results_its[results_its$instance_name=='esc32b',]$best_known_solution_value = 168
# results_sa [results_sa $instance_name=='esc32b',]$best_known_solution_value = 168
#
# results_bma[results_bma$instance_name=='esc32c',]$best_known_solution_value = 642
# results_ea [results_ea $instance_name=='esc32c',]$best_known_solution_value = 642
# results_its[results_its$instance_name=='esc32c',]$best_known_solution_value = 642
# results_sa [results_sa $instance_name=='esc32c',]$best_known_solution_value = 642
#
# results_bma[results_bma$instance_name=='esc32d',]$best_known_solution_value = 200
# results_ea [results_ea $instance_name=='esc32d',]$best_known_solution_value = 200
# results_its[results_its$instance_name=='esc32d',]$best_known_solution_value = 200
# results_sa [results_sa $instance_name=='esc32d',]$best_known_solution_value = 200
#
# results_bma[results_bma$instance_name=='esc32h',]$best_known_solution_value = 438
# results_ea [results_ea $instance_name=='esc32h',]$best_known_solution_value = 438
# results_its[results_its$instance_name=='esc32h',]$best_known_solution_value = 438
# results_sa [results_sa $instance_name=='esc32h',]$best_known_solution_value = 438
#
# results_bma[results_bma$instance_name=='esc64a',]$best_known_solution_value = 116
# results_ea [results_ea $instance_name=='esc64a',]$best_known_solution_value = 116
# results_its[results_its$instance_name=='esc64a',]$best_known_solution_value = 116
# results_sa [results_sa $instance_name=='esc64a',]$best_known_solution_value = 116
#
# results_bma[results_bma$instance_name=='tai100b',]$best_known_solution_value = 1185996137
# results_ea [results_ea $instance_name=='tai100b',]$best_known_solution_value = 1185996137
# results_its[results_its$instance_name=='tai100b',]$best_known_solution_value = 1185996137
# results_sa [results_sa $instance_name=='tai100b',]$best_known_solution_value = 1185996137

results_bma[results_bma$instance_name=='tai10a',]$best_known_solution_value = 135028
results_ea [results_ea $instance_name=='tai10a',]$best_known_solution_value = 135028
results_its[results_its$instance_name=='tai10a',]$best_known_solution_value = 135028
results_sa [results_sa $instance_name=='tai10a',]$best_known_solution_value = 135028

results_bma[results_bma$instance_name=='tai10b',]$best_known_solution_value = 1183760
results_ea [results_ea $instance_name=='tai10b',]$best_known_solution_value = 1183760
results_its[results_its$instance_name=='tai10b',]$best_known_solution_value = 1183760
results_sa [results_sa $instance_name=='tai10b',]$best_known_solution_value = 1183760

# results_bma[results_bma$instance_name=='tai35b',]$best_known_solution_value = 283315445
# results_ea [results_ea $instance_name=='tai35b',]$best_known_solution_value = 283315445
# results_its[results_its$instance_name=='tai35b',]$best_known_solution_value = 283315445
# results_sa [results_sa $instance_name=='tai35b',]$best_known_solution_value = 283315445
#
# results_bma[results_bma$instance_name=='tai40b',]$best_known_solution_value = 637250948
# results_ea [results_ea $instance_name=='tai40b',]$best_known_solution_value = 637250948
# results_its[results_its$instance_name=='tai40b',]$best_known_solution_value = 637250948
# results_sa [results_sa $instance_name=='tai40b',]$best_known_solution_value = 637250948
#
# results_bma[results_bma$instance_name=='tai80b',]$best_known_solution_value = 818415043
# results_ea [results_ea $instance_name=='tai80b',]$best_known_solution_value = 818415043
# results_its[results_its$instance_name=='tai80b',]$best_known_solution_value = 818415043
# results_sa [results_sa $instance_name=='tai80b',]$best_known_solution_value = 818415043

results_bma$deviation <- (results_bma$soln_value - results_bma$best_known_solution_value) / results_bma$best_known_solution_value
results_ea $deviation <- (results_ea $soln_value - results_ea $best_known_solution_value) / results_ea $best_known_solution_value
results_its$deviation <- (results_its$soln_value - results_its$best_known_solution_value) / results_its$best_known_solution_value
results_sa $deviation <- (results_sa $soln_value - results_sa $best_known_solution_value) / results_sa $best_known_solution_value

bkv_rate_bma <- sum(results_bma$soln_value <= results_bma$best_known_solution_value)/length(results_bma$best_known_solution_value)
bkv_rate_ea  <- sum(results_ea $soln_value <= results_ea $best_known_solution_value)/length(results_ea $best_known_solution_value)
bkv_rate_its <- sum(results_its$soln_value <= results_its$best_known_solution_value)/length(results_its$best_known_solution_value)
bkv_rate_sa  <- sum(results_sa $soln_value <= results_sa $best_known_solution_value)/length(results_sa $best_known_solution_value)

deviation_bma <- aggregate(results_bma$deviation, by=list(results_bma$instance_name), FUN=mean)$x
deviation_ea  <- aggregate(results_ea $deviation, by=list(results_ea $instance_name), FUN=mean)$x
deviation_its <- aggregate(results_its$deviation, by=list(results_its$instance_name), FUN=mean)$x
deviation_sa  <- aggregate(results_sa $deviation, by=list(results_sa $instance_name), FUN=mean)$x

time_bma <- aggregate(results_bma$time_to_best_solution_seconds, by=list(results_bma$instance_name), FUN=mean)$x
time_ea  <- aggregate(results_ea $time_to_best_solution_seconds, by=list(results_ea $instance_name), FUN=mean)$x
time_its <- aggregate(results_its$time_to_best_solution_seconds, by=list(results_its$instance_name), FUN=mean)$x
time_sa  <- aggregate(results_sa $time_to_best_solution_seconds, by=list(results_sa $instance_name), FUN=mean)$x

sorted_soln_value_bma <- aggregate(results_bma$soln_value, by=list(results_bma$instance_name), FUN=mean)$x
sorted_soln_value_ea  <- aggregate(results_ea $soln_value, by=list(results_ea $instance_name), FUN=mean)$x
sorted_soln_value_its <- aggregate(results_its$soln_value, by=list(results_its$instance_name), FUN=mean)$x
sorted_soln_value_sa  <- aggregate(results_sa $soln_value, by=list(results_sa $instance_name), FUN=mean)$x

sorted_bkv_bma <- aggregate(results_bma$best_known_solution_value, by=list(results_bma$instance_name), FUN=mean)$x
sorted_bkv_ea  <- aggregate(results_ea $best_known_solution_value, by=list(results_ea $instance_name), FUN=mean)$x
sorted_bkv_its <- aggregate(results_its$best_known_solution_value, by=list(results_its$instance_name), FUN=mean)$x
sorted_bkv_sa  <- aggregate(results_sa $best_known_solution_value, by=list(results_sa $instance_name), FUN=mean)$x

bkv_indices_bma <- sorted_soln_value_bma <= sorted_bkv_bma
bkv_indices_ea  <- sorted_soln_value_ea  <= sorted_bkv_ea
bkv_indices_its <- sorted_soln_value_its <= sorted_bkv_its
bkv_indices_sa  <- sorted_soln_value_sa  <= sorted_bkv_sa

avg_bkv_times_bma <- mean(time_bma[bkv_indices_bma])
avg_bkv_times_ea  <- mean(time_ea [bkv_indices_ea ])
avg_bkv_times_its <- mean(time_its[bkv_indices_its])
avg_bkv_times_sa  <- mean(time_sa [bkv_indices_sa ])

bkv_time_bma <- mean(results_bma[results_bma$soln_value <= results_bma$best_known_solution_value,]$time_to_best_solution_seconds)
bkv_time_ea  <- mean(results_ea [results_ea $soln_value <= results_ea $best_known_solution_value,]$time_to_best_solution_seconds)
bkv_time_its <- mean(results_its[results_its$soln_value <= results_its$best_known_solution_value,]$time_to_best_solution_seconds)
bkv_time_sa  <- mean(results_sa [results_sa $soln_value <= results_sa $best_known_solution_value,]$time_to_best_solution_seconds)

sorted_names <- aggregate(results_bma$deviation, by=list(results_bma$instance_name), FUN=mean)
sorted_sizes <- as.numeric(gsub("\\D", "", sorted_names$Group.1))
small_indices <- sorted_sizes <= 64
large_indices <- sorted_sizes > 64

results_mat <- matrix(
    c( deviation_bma #results_bma$deviation
     , deviation_ea #results_ea $deviation
     , deviation_its #results_its$deviation
     , deviation_sa
     ), #results_sa $deviation),
    nrow=length(deviation_bma), #length(results_bma$soln_value),
    #byrow = TRUE,
    dimnames = list(1 : length(deviation_bma), #length(results_bma$soln_value),
                    c("BMA", "EA", "ITS", "SA"))
)

results_times_mat <- matrix(
    c( time_bma #results_bma$deviation
     , time_ea #results_ea $deviation
     , time_its #results_its$deviation
     , time_sa
     ), #results_sa $deviation),
    nrow=length(time_bma), #length(results_bma$soln_value),
    #byrow = TRUE,
    dimnames = list(1 : length(time_bma), #length(results_bma$soln_value),
                    c("BMA", "EA", "ITS", "SA"))
)

##### DEVIATION PLOTS: #####

pdf("results-deviations-box-plot.pdf",width=5.5,height=4.5)
par(mfrow=c(1,1))
par(mar=c(4,4,3,1))
boxplot(results_mat*100, main="Average deviations for all QAPLIB instances",
        xlab="Algorithm", ylab="Average deviation (%)", range=1.5, ylim=c(0,30)
        )
abline(h=mean(results_mat*100), col="red")
points(colMeans(results_mat*100), col="red",pch=18)
dev.off()

results_mat_small = results_mat[small_indices,]
pdf("results-deviations-small-box-plot.pdf",width=5.5,height=4.5)
par(mfrow=c(1,1))
par(mar=c(4,4,3,1))
boxplot(results_mat_small*100, main="Average deviations for small QAPLIB instances",
        xlab="Algorithm", ylab="Average deviation (%)", range=1.5, ylim=c(0,30)
        )
abline(h=mean(results_mat_small*100), col="red")
points(colMeans(results_mat_small*100), col="red",pch=18)
dev.off()


results_mat_large = results_mat[large_indices,]
pdf("results-deviations-large-box-plot.pdf",width=5.5,height=4.5)
par(mfrow=c(1,1))
par(mar=c(4,4,3,1))
boxplot(results_mat_large*100, main="Average deviations for large QAPLIB instances",
        xlab="Algorithm", ylab="Average deviation (%)", range=1.5, ylim=c(0,30)
        )
abline(h=mean(results_mat_large*100), col="red")
points(colMeans(results_mat_large*100), col="red",pch=18)
dev.off()


##### TIME PLOTS: #####

pdf("results-times-box-plot.pdf",width=5.5,height=4.5)
par(mfrow=c(1,1))
par(mar=c(4,4,3,1))
boxplot(results_times_mat, main="Average times for all QAPLIB instances",
        xlab="Algorithm", ylab="Average time (s)", range=1.5, ylim=c(0,15)
        )
abline(h=mean(results_times_mat), col="red")
points(colMeans(results_times_mat), col="red",pch=18)
dev.off()

results_times_mat_small = results_times_mat[small_indices,]
pdf("results-times-small-box-plot.pdf",width=5.5,height=4.5)
par(mfrow=c(1,1))
par(mar=c(4,4,3,1))
boxplot(results_times_mat_small, main="Average times for small QAPLIB instances",
        xlab="Algorithm", ylab="Average time (s)", range=1.5, ylim=c(0,15)
        )
abline(h=mean(results_times_mat_small), col="red")
points(colMeans(results_times_mat_small), col="red",pch=18)
dev.off()


results_times_mat_large = results_times_mat[large_indices,]
pdf("results-times-large-box-plot.pdf",width=5.5,height=4.5)
par(mfrow=c(1,1))
par(mar=c(4,4,3,1))
boxplot(results_times_mat_large, main="Average times for large QAPLIB instances",
        xlab="Algorithm", ylab="Average time (s)", range=1.5, ylim=c(0,15)
        )
abline(h=mean(results_times_mat_large), col="red")
points(colMeans(results_times_mat_large), col="red",pch=18)
dev.off()

##### SIGNIFICANCE TESTS: #####

test_result <- friedman.test(results_mat)

wilcox.test(deviation_bma, deviation_its, paired = TRUE, alternative = "greater")
wilcox.test(deviation_bma, deviation_ea , paired = TRUE, alternative = "greater")
wilcox.test(deviation_bma, deviation_sa , paired = TRUE, alternative = "greater")
wilcox.test(deviation_its, deviation_ea , paired = TRUE, alternative = "greater")
wilcox.test(deviation_its, deviation_sa , paired = TRUE, alternative = "greater")
wilcox.test(deviation_ea , deviation_sa , paired = TRUE, alternative = "greater")
