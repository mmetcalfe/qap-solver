results_bma <- read.table("results_bma.tsv",header=T)
results_ea <- read.table("results_ea.tsv",header=T)
results_its <- read.table("results_its.tsv",header=T)
results_sa <- read.table("results_sa.tsv",header=T)

results_bma[results_bma$instance_name=='esc32a',]$best_known_solution_value = 130
results_ea [results_ea $instance_name=='esc32a',]$best_known_solution_value = 130
results_its[results_its$instance_name=='esc32a',]$best_known_solution_value = 130
results_sa [results_sa $instance_name=='esc32a',]$best_known_solution_value = 130

results_bma[results_bma$instance_name=='esc32b',]$best_known_solution_value = 168
results_ea [results_ea $instance_name=='esc32b',]$best_known_solution_value = 168
results_its[results_its$instance_name=='esc32b',]$best_known_solution_value = 168
results_sa [results_sa $instance_name=='esc32b',]$best_known_solution_value = 168

results_bma[results_bma$instance_name=='esc32c',]$best_known_solution_value = 642
results_ea [results_ea $instance_name=='esc32c',]$best_known_solution_value = 642
results_its[results_its$instance_name=='esc32c',]$best_known_solution_value = 642
results_sa [results_sa $instance_name=='esc32c',]$best_known_solution_value = 642

results_bma[results_bma$instance_name=='esc32d',]$best_known_solution_value = 200
results_ea [results_ea $instance_name=='esc32d',]$best_known_solution_value = 200
results_its[results_its$instance_name=='esc32d',]$best_known_solution_value = 200
results_sa [results_sa $instance_name=='esc32d',]$best_known_solution_value = 200

results_bma[results_bma$instance_name=='esc32h',]$best_known_solution_value = 438
results_ea [results_ea $instance_name=='esc32h',]$best_known_solution_value = 438
results_its[results_its$instance_name=='esc32h',]$best_known_solution_value = 438
results_sa [results_sa $instance_name=='esc32h',]$best_known_solution_value = 438

results_bma[results_bma$instance_name=='esc64a',]$best_known_solution_value = 116
results_ea [results_ea $instance_name=='esc64a',]$best_known_solution_value = 116
results_its[results_its$instance_name=='esc64a',]$best_known_solution_value = 116
results_sa [results_sa $instance_name=='esc64a',]$best_known_solution_value = 116

results_bma[results_bma$instance_name=='tai100b',]$best_known_solution_value = 1185996137
results_ea [results_ea $instance_name=='tai100b',]$best_known_solution_value = 1185996137
results_its[results_its$instance_name=='tai100b',]$best_known_solution_value = 1185996137
results_sa [results_sa $instance_name=='tai100b',]$best_known_solution_value = 1185996137

results_bma[results_bma$instance_name=='tai10a',]$best_known_solution_value = 135028
results_ea [results_ea $instance_name=='tai10a',]$best_known_solution_value = 135028
results_its[results_its$instance_name=='tai10a',]$best_known_solution_value = 135028
results_sa [results_sa $instance_name=='tai10a',]$best_known_solution_value = 135028

results_bma[results_bma$instance_name=='tai10b',]$best_known_solution_value = 1183760
results_ea [results_ea $instance_name=='tai10b',]$best_known_solution_value = 1183760
results_its[results_its$instance_name=='tai10b',]$best_known_solution_value = 1183760
results_sa [results_sa $instance_name=='tai10b',]$best_known_solution_value = 1183760

results_bma[results_bma$instance_name=='tai35b',]$best_known_solution_value = 283315445
results_ea [results_ea $instance_name=='tai35b',]$best_known_solution_value = 283315445
results_its[results_its$instance_name=='tai35b',]$best_known_solution_value = 283315445
results_sa [results_sa $instance_name=='tai35b',]$best_known_solution_value = 283315445

results_bma[results_bma$instance_name=='tai40b',]$best_known_solution_value = 637250948
results_ea [results_ea $instance_name=='tai40b',]$best_known_solution_value = 637250948
results_its[results_its$instance_name=='tai40b',]$best_known_solution_value = 637250948
results_sa [results_sa $instance_name=='tai40b',]$best_known_solution_value = 637250948

results_bma[results_bma$instance_name=='tai80b',]$best_known_solution_value = 818415043
results_ea [results_ea $instance_name=='tai80b',]$best_known_solution_value = 818415043
results_its[results_its$instance_name=='tai80b',]$best_known_solution_value = 818415043
results_sa [results_sa $instance_name=='tai80b',]$best_known_solution_value = 818415043

# logisticRegressionPlot <- function(trialDataPrefix, xName, yName, filterName, filterValue, xLabel, yLabel) {
#     library(locfit)
#     myexpit <- function(x,b0,b1){expit <- exp(b0+b1*x)/( 1+exp(b0+b1*x) )}
#     #bw <- read.table("birthweight.txt",header=T)
#     bw <- read.table(paste(trialDataPrefix,".tsv", sep=""),header=T)#read.table(trialDataFile,header=T)
#     bw$filterCol <- bw[,filterName]
#     bw <- subset(bw, filterCol==filterValue)
#     # bw <- bw[seq(1,100),]
#     #bw$safeResult <- bw$safeResult*100
#     #bw$fallResult <- bw$fallResult*100
#     #bw$ambiguousResult <- bw$ambiguousResult*100
#     #bw$timeout <- (1-bw$kickFailure) * (1-bw$collisionFailure) * (1-bw$kickSuccess)
#     #bw$timeout <- (1-bw$collisionFailure) * (1-bw$kickSuccess)
#     birthweight <- bw$birthweight <- data.matrix(bw[xName])
#     BPD <- bw$BPD <- data.matrix(bw[yName])
#     #birthweight <- bw$birthweight <- bw$chanceConstraint
#     #BPD <- bw$BPD <- (1-bw$kickFailure) * (1-bw$collisionFailure) * (1-bw$kickSuccess)
#     #BPD <- bw$BPD <- (1-kickFailure) * (1-collisionFailure) * (1-kickSuccess)
#     #BPD <- bw$BPD <- bw$kickFailure
#     #BPD <- bw$BPD <- bw$kickSuccess
#     #BPD <- bw$BPD <- bw$collisionFailure
#     #attach(bw)
#     lrmod1 <- glm(BPD~birthweight,family=binomial)
#     x <- seq(min(birthweight),max(birthweight))
#     y <- bw$BPD
#     meanbw <- mean(bw$birthweight)
#     sdbw <- sd(bw$birthweight)
#     x <- (bw$birthweight-meanbw)/sdbw
#     #
#     # My own AIC function
#     #
#     clamp <- function(vals,minval,maxval) {
#       minvals <- rep(minval,length(vals))
#       maxvals <- rep(maxval,length(vals))
#       result <- vals
#
#       ind <- which(vals < minvals)
#       result[ind] = minvals[ind]
#
#       ind <- which(vals > maxvals)
#       result[ind] = maxvals[ind]
#
#       result
#     }
#     aicbinfun <- function(y,nl,p,df){
#       probs <- clamp(p, 1e-6, 1e6)
#       compls <- clamp(1 - p, 1e-6, 1e6)
#       ndat <- length(y)
#       dfterm <- 2*df
#       # loglik <- 0
#       # for (i in 1:ndat){
#       #   logliki <- lchoose(nl[i],y[i])
#       #            + y[i]*log(probs[i])
#       #            + (nl[i]-y[i])*log(compls)
#       #   loglik <- loglik + logliki
#       # }
#       loglik <- sum(lchoose(nl,y)+y*log(probs)+(nl-y)*log(compls))
#       aicbin <- (-2*loglik  + dfterm)
#       list(aicbin=aicbin,dfterm=dfterm,loglik=loglik)
#     }
#     n <- rep(1,length(y))
#     hval <- seq(1,5,.1) # seq(1,5,.1)
#     aicval <- myaicval <- df <- loglik <- NULL
#     for (i in 1:length(hval)){
#       locfitbw <- locfit(y~lp(x,deg=1,nn=0,h=hval[i]),data=bw,family="binomial")
#       preds <- predict(locfitbw,newdata=x)
#       df[i] <- locfitbw$dp['df1'] # locfitbw[9]$dp[6]
#       myaicval[i] <- aicbinfun(y,n,preds,df[i])$aicbin
#       loglik[i] <- aicbinfun(y,n,preds,df[i])$loglik
#     }
#     par(mfrow=c(1,2))
#     plot(myaicval~hval)
#     plot(df~hval)
#     hopt <- hval[which.min(myaicval)]
#     dfopt <- df[which.min(myaicval)]
#     cat("Minimizing values h and df =",hopt,dfopt,"\n") # df=4.1
#     #
#     # Now obtain the fit with the minimizing value of h
#     #
#     locgcv <- locfit(y~lp(x,deg=1,nn=0,h=hopt),data=bw,family="binomial")
#     xseq <- seq(min(x),max(x),.01)
#     preds <- predict(locgcv,newdata=xseq)
#     newx <- xseq*sdbw + meanbw
#     #
#     # Penalized regression splines for the BPD data
#     #
#     library(mgcv)
#     #attach(bw)
#     #
#     gammod <- gam(bw$BPD~s(x=bw$birthweight,k=10,fx=F,bs="cr",m=2),family=binomial)
#     orderbw <- cbind(BPD,birthweight)
#     orderbw <- orderbw[order(orderbw[,2]),]
#     orderfit <- cbind(birthweight,gammod$fitted)
#     orderfit <- orderfit[order(orderfit[,1]),]
#     #
#     # Fig 11.16
#     #
#     pdf(paste(trialDataPrefix,"-",yName,"-",filterValue,".pdf", sep=""),width=4.5,height=4.5)
#     par(mfrow=c(1,1))
#     par(mar=c(4,4,1,1))
#     plot(birthweight,BPD,pch="|",xlab=xLabel,ylab=yLabel, axes=FALSE)
#
#     axis(1)
#     pts <- pretty(seq(0,100,10))
#     axis(2, at = pts/100, labels = paste(pts, "", sep = ""))
#     box()
#
#     #plot(birthweight[1:200],BPD[1:200],pch="|",xlab="Birthweight (grams)")
#     lines(orderfit[,1],orderfit[,2],lty=1)
#     lrmod1 <- glm(BPD~birthweight,family=binomial)
#     xseq2 <- seq(min(birthweight),max(birthweight), 0.01)
#     #myexpit <- function(x,b0,b1){exp(b0+b1*x)/( 1+exp(b0+b1*x) )}
#     myexpit <- function(x,b0,b1){expit <- exp(b0+b1*x)/( 1+exp(b0+b1*x) )}
#     lines(xseq2,myexpit(xseq2,b0=lrmod1$coeff[1],b1=lrmod1$coeff[2]),lty=3)
#     lines(newx,preds,lty=2)
#     legend(x=-5.5,y=0.6,legend=c("Cubic Spline","Local Likelihood","Linear Logistic"),
#            bty="n",lty=1:3)
#     dev.off()
# }
#
#
# finishTimesPlot <- function(trialDataPrefix, xName, xLabel, xLim) {
#   yName <- 'finishTime'
#   yLabel <- 'Finish Time (s)'
#   rawdata <- read.table(paste(trialDataPrefix,".tsv", sep=""),header=T)
#   #rawdata$kickSuccess <- rawdata$kickSuccess*(rawdata$numAlmostKicks==0)
#   rawdata$xCol <- rawdata[,xName]
#   rawdata$yCol <- rawdata[,yName]
#   #validdata <- subset(rawdata, safeResult!=1)
#   #validdata <- subset(rawdata, ambiguousResult==1)
#   validdata <- subset(rawdata, fallResult==1)
#
#   model <- glm(formula=yCol~xCol, data=validdata, family=gaussian)
#
#   xPreds <- seq(xLim[1],xLim[2], 0.01)
#   yPreds <- predict(model, newdata=list(xCol = xPreds), type="response")
#
#   pdf(paste(trialDataPrefix,"-",yName,".pdf", sep=""),width=4.5,height=4.5)
#   par(mfrow=c(1,1))
#   #plot(rawdata$xCol,rawdata$yCol,pch="|",xlab=xLabel,ylab=yLabel)
#   par(mar=c(4,4,1,1))
#   plot(validdata$xCol,validdata$yCol,xlab=xLabel,ylab=yLabel, xlim=xLim, ylim=c(0, 7))
#   #plot(validdata$xCol,validdata$yCol,xlab=xLabel,ylab=yLabel, xlim=xLim)
#   # lines(xPreds, yPreds, col='red')
#
#   #plot(birthweight[1:200],BPD[1:200],pch="|",xlab="Birthweight (grams)")
#   #lines(orderfit[,1],orderfit[,2],lty=1)
#   #lrmod1 <- glm(BPD~birthweight,family=binomial)
#   #xseq2 <- seq(min(birthweight),max(birthweight), 0.01)
#   #myexpit <- function(x,b0,b1){exp(b0+b1*x)/( 1+exp(b0+b1*x) )}
#   #lines(newx,preds,lty=2)
#   #legend(x=0.3,y=0.75,legend=c("Cubic Spline","Local Likelihood","Linear Logistic"), bty="n",lty=1:3)
#   dev.off()
#
#   model
# }
#
# summary(finishTimesPlot("results", "pushForce", "Push Force (N)", c(-5, 5)))
#
# logisticRegressionPlot("results", "pushForce", "safeResult", "pushDuration", 0.2, "Push Force (N)", "Safe Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "ambiguousResult", "pushDuration", 0.2, "Push Force (N)", "Ambiguous Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "fallResult", "pushDuration", 0.2, "Push Force (N)", "Fall Result Rate (%)")
#
# logisticRegressionPlot("results", "pushForce", "safeResult", "pushStartPhase", "Push", "Push Force (N)", "Safe Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "ambiguousResult", "pushStartPhase", "Push", "Push Force (N)", "Ambiguous Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "fallResult", "pushStartPhase", "Push", "Push Force (N)", "Fall Result Rate (%)")
#
# logisticRegressionPlot("results", "pushForce", "safeResult", "pushStartPhase", "Drop", "Push Force (N)", "Safe Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "ambiguousResult", "pushStartPhase", "Drop", "Push Force (N)", "Ambiguous Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "fallResult", "pushStartPhase", "Drop", "Push Force (N)", "Fall Result Rate (%)")
#
# logisticRegressionPlot("results", "pushForce", "safeResult", "pushStartPhase", "Lift", "Push Force (N)", "Safe Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "ambiguousResult", "pushStartPhase", "Lift", "Push Force (N)", "Ambiguous Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "fallResult", "pushStartPhase", "Lift", "Push Force (N)", "Fall Result Rate (%)")
#
# logisticRegressionPlot("results", "pushForce", "safeResult", "pushStartPhase", "Swing", "Push Force (N)", "Safe Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "ambiguousResult", "pushStartPhase", "Swing", "Push Force (N)", "Ambiguous Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "fallResult", "pushStartPhase", "Swing", "Push Force (N)", "Fall Result Rate (%)")
#
# logisticRegressionPlot("results", "pushForce", "safeResult", "pushStartPhase", "StandStill", "Push Force (N)", "Safe Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "ambiguousResult", "pushStartPhase", "StandStill", "Push Force (N)", "Ambiguous Result Rate (%)")
# logisticRegressionPlot("results", "pushForce", "fallResult", "pushStartPhase", "StandStill", "Push Force (N)", "Fall Result Rate (%)")